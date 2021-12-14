use kernel_micro::{include_file, MicroKernel};

/// A microkernel for R
pub fn new() -> MicroKernel {
    MicroKernel::new(
        "r-micro",
        &["r"],
        &["linux", "macos", "windows"],
        &["linux", "macos"],
        ("Rscript", "*"),
        &["{{script}}"],
        include_file!("r-kernel.r"),
        &[include_file!("r-codec.r")],
        "{{name}} <- decode_value(\"{{json}}\")",
        "{{name}}",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use kernel::{
        eyre::{bail, Result},
        stencila_schema::Node,
        KernelTrait,
    };
    use once_cell::sync::Lazy;
    use test_utils::{assert_json_eq, serde_json::json};
    use tokio::sync::Mutex;

    // Make sure that only one test runs at any one time
    // This is to avoid `install.packages` being run in parallel.
    // Previous the `serial_test` crate was used for this but did not
    // seem to provide necessary guarantee.
    static QUEUE: Lazy<Mutex<()>> = Lazy::new(Mutex::default);

    async fn skip_or_kernel() -> Result<MicroKernel> {
        let mut kernel = new();
        if !kernel.available().await {
            eprintln!("R not available on this machine");
            bail!("Skipping")
        } else {
            kernel.start().await?;
        }

        Ok(kernel)
    }

    // Run these tests serially to avoid parallel installation of dependencies
    // which may interfere with each other.

    /// Tests of basic functionality
    /// This test is replicated in all the microkernels.
    /// Other test should be written for language specific quirks and regressions.
    #[tokio::test]
    async fn basics() -> Result<()> {
        let _guard = QUEUE.lock().await;

        let mut kernel = match skip_or_kernel().await {
            Ok(kernel) => kernel,
            Err(..) => return Ok(()),
        };

        // Assign a variable and output it
        let (outputs, messages) = kernel.exec("a = 2\na").await?;
        assert_json_eq!(messages, json!([]));
        assert_json_eq!(outputs, [[2]]);

        // Print the variable twice and then output it
        let (outputs, messages) = kernel.exec("print(a)\nprint(a)\na").await?;
        assert_json_eq!(messages, json!([]));
        assert_json_eq!(outputs, [[2], [2], [2]]);

        // Syntax error
        let (outputs, messages) = kernel.exec("bad ^ # syntax").await?;
        assert_json_eq!(messages[0].error_type, "SyntaxError");
        assert_json_eq!(
            messages[0].error_message,
            "<text>:2:0: unexpected end of input\n1: bad ^ # syntax\n   ^"
        );
        assert_json_eq!(outputs, json!([]));

        // Runtime error
        let (outputs, messages) = kernel.exec("foo").await?;
        assert_json_eq!(messages[0].error_type, "RuntimeError");
        assert_json_eq!(messages[0].error_message, "object 'foo' not found");
        assert_json_eq!(outputs, json!([]));

        // Set and get another variable
        kernel.set("b", Node::Integer(3)).await?;
        let b = kernel.get("b").await?;
        assert_json_eq!(b, [3]);

        // Use both variables
        let (outputs, messages) = kernel.exec("a*b").await?;
        assert_json_eq!(messages, json!([]));
        assert_json_eq!(outputs, [[6]]);

        Ok(())
    }

    /// Test that an assignment on the last line does not generate an output
    #[tokio::test]
    async fn assignment_no_output() -> Result<()> {
        let _guard = QUEUE.lock().await;

        let mut kernel = match skip_or_kernel().await {
            Ok(kernel) => kernel,
            Err(..) => return Ok(()),
        };

        let (outputs, messages) = kernel.exec("a <- 1").await?;
        assert!(messages.is_empty());
        assert_json_eq!(outputs, json!([]));

        let (outputs, messages) = kernel.exec("b = 2").await?;
        assert!(messages.is_empty());
        assert_json_eq!(outputs, json!([]));

        let (outputs, messages) = kernel.exec("print(a)\nprint(b)\na_b <- a + b").await?;
        assert!(messages.is_empty());
        assert_json_eq!(outputs, [[1], [2]]);

        Ok(())
    }

    #[tokio::test]
    async fn encode_general() -> Result<()> {
        let _guard = QUEUE.lock().await;

        let mut kernel = match skip_or_kernel().await {
            Ok(kernel) => kernel,
            Err(..) => return Ok(()),
        };

        // Null, booleans, integers, numbers, strings
        let (outputs, messages) = kernel
            .exec("list(NULL, TRUE, FALSE, 1, 1.23456789, 'str')")
            .await?;
        assert_json_eq!(messages, json!([]));
        assert_json_eq!(
            outputs,
            json!([[null, [true], [false], [1], [1.23456789], ["str"]]])
        );

        // Arrays
        let (outputs, messages) = kernel.exec("1:5").await?;
        assert_json_eq!(messages, json!([]));
        assert_json_eq!(outputs, [[1, 2, 3, 4, 5]]);

        // Objects
        let (outputs, messages) = kernel.exec("list(a=1, b=list(c=2))").await?;
        assert_json_eq!(messages, json!([]));
        assert_json_eq!(outputs, json!([{"type": "Entity"}]));
        // TODO: correct output when deserialization of Node::Object is working
        //assert_json_eq!(outputs, json!([{"a": [1], "b": {"c": [2]}}]));

        // Matrix
        let (outputs, messages) = kernel.exec("matrix(c(1:4), 2, 2)").await?;
        assert_json_eq!(messages, json!([]));
        assert_json_eq!(outputs, [[[1, 3], [2, 4]]]);

        Ok(())
    }

    #[tokio::test]
    async fn encode_dataframes() -> Result<()> {
        let _guard = QUEUE.lock().await;

        let mut kernel = match skip_or_kernel().await {
            Ok(kernel) => kernel,
            Err(..) => return Ok(()),
        };

        let (outputs, messages) = kernel
            .exec(
                r#"data.frame(
    a = 1:2,
    b = c(TRUE, FALSE),
    c = c("x", "y"),
    d = factor(c("X", "Y"), levels = c("X", "Y", "Z")),
    stringsAsFactors = FALSE
)"#,
            )
            .await?;
        assert_json_eq!(messages, json!([]));
        let dt = match &outputs[0] {
            Node::Datatable(dt) => dt.clone(),
            _ => bail!("unexpected type {:?}", outputs[0]),
        };
        assert_eq!(
            dt.columns
                .iter()
                .map(|column| column.name.as_str())
                .collect::<Vec<&str>>(),
            vec!["a", "b", "c", "d"]
        );
        assert_json_eq!(
            dt.columns[0].validator.as_ref().unwrap().items_validator,
            json!({ "type": "NumberValidator"})
        );
        assert_json_eq!(
            dt.columns[1].validator.as_ref().unwrap().items_validator,
            json!({ "type": "BooleanValidator"})
        );
        assert_json_eq!(
            dt.columns[2].validator.as_ref().unwrap().items_validator,
            json!({ "type": "StringValidator"})
        );
        assert_json_eq!(
            dt.columns[3].validator.as_ref().unwrap().items_validator,
            json!({
                "type": "EnumValidator",
                "values": ["X", "Y", "Z"]
            })
        );

        let (outputs, messages) = kernel.exec("mtcars").await?;
        assert_json_eq!(messages, json!([]));
        let dt = match &outputs[0] {
            Node::Datatable(dt) => dt.clone(),
            _ => bail!("unexpected type {:?}", outputs[0]),
        };
        assert_eq!(
            dt.columns
                .iter()
                .map(|column| column.name.as_str())
                .collect::<Vec<&str>>(),
            vec![
                "name", "mpg", "cyl", "disp", "hp", "drat", "wt", "qsec", "vs", "am", "gear",
                "carb"
            ]
        );
        assert_json_eq!(
            dt.columns[0].validator.as_ref().unwrap().items_validator,
            json!({ "type": "StringValidator"})
        );
        assert_json_eq!(
            dt.columns[1].validator.as_ref().unwrap().items_validator,
            json!({ "type": "NumberValidator"})
        );

        let (outputs, messages) = kernel.exec("chickwts").await?;
        assert_json_eq!(messages, json!([]));
        let dt = match &outputs[0] {
            Node::Datatable(dt) => dt.clone(),
            _ => bail!("unexpected type {:?}", outputs[0]),
        };
        assert_eq!(
            dt.columns
                .iter()
                .map(|column| column.name.as_str())
                .collect::<Vec<&str>>(),
            vec!["weight", "feed"]
        );
        assert_json_eq!(
            dt.columns[0].validator.as_ref().unwrap().items_validator,
            json!({ "type": "NumberValidator"})
        );
        assert_json_eq!(
            dt.columns[1].validator.as_ref().unwrap().items_validator,
            json!({
                "type": "EnumValidator",
                "values": ["casein", "horsebean", "linseed", "meatmeal", "soybean", "sunflower"]
            })
        );

        Ok(())
    }

    #[tokio::test]
    async fn encode_plots() -> Result<()> {
        let _guard = QUEUE.lock().await;

        let mut kernel = match skip_or_kernel().await {
            Ok(kernel) => kernel,
            Err(..) => return Ok(()),
        };

        for code in ["plot(1)", "hist(rnorm(1000), breaks=30)"] {
            let (outputs, messages) = kernel.exec(code).await?;
            assert_json_eq!(messages, json!([]));
            let image = match &outputs[0] {
                Node::ImageObject(dt) => dt.clone(),
                _ => bail!("unexpected type {:?}", outputs[0]),
            };
            assert!(image.content_url.starts_with("data:image/png;base64,"));
        }

        Ok(())
    }

    /// Test forking
    #[tokio::test]
    async fn fork() -> Result<()> {
        let _guard = QUEUE.lock().await;

        let mut kernel = match skip_or_kernel().await {
            Ok(kernel) => {
                if kernel.forkable().await {
                    kernel
                } else {
                    eprintln!("Not forkable on this OS");
                    return Ok(());
                }
            }
            Err(..) => return Ok(()),
        };

        // In the kernel import a module and assign a variable
        let (outputs, messages) = kernel.exec("var = runif(1)\nvar").await?;
        assert_json_eq!(messages, json!([]));
        assert_eq!(outputs.len(), 1);
        let var = outputs[0].clone();

        // Now fork-exec. The fork should be able to use the module and access the
        // variable but any change to variable should not change its value in the parent kernel
        let (outputs, messages) = kernel.fork_exec("print(var)\nvar = runif(1)").await?;
        assert_json_eq!(messages, json!([]));
        assert_eq!(outputs.len(), 1);
        assert_json_eq!(outputs[0], var);

        // Back in the parent kernel, var should still have its original value
        assert_json_eq!(var, kernel.get("var").await?);
        let (outputs, messages) = kernel.exec("var").await?;
        assert_json_eq!(messages, json!([]));
        assert_eq!(outputs.len(), 1);

        Ok(())
    }
}