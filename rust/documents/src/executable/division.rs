use common::async_trait::async_trait;

use stencila_schema::Division;

use super::Executable;

#[async_trait]
impl Executable for Division {
    /// Compile a `Division` node
    #[cfg(ignore)]
    async fn compile(&self, address: &mut Address, context: &mut CompileContext) -> Result<()> {
        let id = ensure_id!(self, "di", context);

        // Compile the content of the division
        self.content.compile(context).await?;

        // Infer the language of the expression, falling back to Tailwind
        let lang = match self.programming_language.is_empty() {
            true => Format::Tailwind,
            false => {
                if (matches!(self.guess_language, Some(true))
                    || self.programming_language.to_lowercase() == "unknown")
                {
                    context
                        .kernel_space
                        .guess_language(&self.text, Format::Tailwind, None, None)
                } else {
                    formats::match_name(&self.programming_language)
                }
            }
        };

        // Generate `ResourceInfo` by parsing the code. If there is a passing error still generate resource info
        // but do not generate errors since the user may still be in the process of writing code
        let resource = resources::code(&context.path, id, "Division", lang);
        let mut resource_info = match parsers::parse(resource.clone(), &self.text) {
            Ok(resource_info) => resource_info,
            Err(..) => ResourceInfo::default(resource),
        };

        // Update the resource info (which has (an incomplete) `compile_digest`) with the `execute_digest` from
        // the last time the code chunk was executed
        resource_info.execute_digest = self.execute_digest.clone();
        resource_info.execute_failed = self.execution_status.as_ref().map(|status| {
            // This function can be called while the node is `Scheduled` so this needs to account for that
            // by considering last execution status as well
            matches!(
                status,
                ExecutionStatus::Failed
                    | ExecutionStatus::ScheduledPreviouslyFailed
                    | ExecutionStatus::RunningPreviouslyFailed
            )
        });

        // Assume side-effect free code expression execution semantics
        resource_info.execute_auto = Some(ExecutionAuto::Always);
        resource_info.execute_pure = Some(true);

        // If the language is Tailwind, and it has not relations (i.e. no variable interpolation) then
        // attempt to transpile it to CSS.
        // Fail silently (do not store errors) since the user may still be in the middle
        // of typing during this compile,
        if matches!(lang, Format::Tailwind)
            && resource_info
                .relations
                .as_ref()
                .map_or_else(|| true, |relations| relations.is_empty())
        {
            if let Ok(css) = parser_tailwind::transpile_string(&self.text) {
                self.css = css;
            }
        }

        context.resource_infos.push(resource_info);

        Ok(())
    }

    /// Begin executing a `Division` node
    ///
    /// Starts an async tak in the kernel space
    #[cfg(ignore)]
    async fn execute_begin(
        &mut self,
        resource_info: &ResourceInfo,
        kernel_space: &KernelSpace,
        kernel_selector: &KernelSelector,
        is_fork: bool,
    ) -> Result<Option<TaskInfo>> {
        let id = assert_id!(self)?;
        tracing::trace!("Executing `Division` `{}`", id);

        let task_info = kernel_space
            .exec(&self.code, resource_info, is_fork, kernel_selector)
            .await?;

        Ok(Some(task_info))
    }

    /// End executing a `Division` node
    ///
    /// Updates various various properties of the node based on the task info and result.
    /// Most importantly, updates the `css` property by transpiling the result of the
    /// evaluation.
    #[cfg(ignore)]
    async fn execute_end(&mut self, task_info: TaskInfo, task_result: TaskResult) -> Result<()> {
        let TaskResult {
            outputs,
            messages: mut errors,
        } = task_result;

        // Update both `compile_digest` and `execute_digest` to the compile digest
        let digest = task_info.resource_info.compile_digest.clone();
        self.compile_digest = digest.clone();
        self.execute_digest = digest;

        // Update execution status, etc
        let execution_status = code_execution_status(&task_info, &errors);
        self.execution_required = Some(if matches!(execution_status, ExecutionStatus::Succeeded) {
            ExecutionRequired::No
        } else {
            ExecutionRequired::Failed
        });
        self.execution_status = Some(execution_status);
        self.execution_ended = task_info
            .ended()
            .map(|ended| Box::new(Timestamp::from(ended)));
        self.execution_duration = task_info
            .duration()
            .map(|duration| Box::new(Duration::from_micros(duration as i64)));
        self.execution_kernel = task_info.kernel_id.map(Box::new);
        self.execution_count = Some(self.execution_count.unwrap_or_default() + 1);

        // Transpile the returned Tailwind string. To avoid unstyled content, if there is
        // an error we do not reset the CSS
        if let Some(node) = outputs.first() {
            match node {
                Node::String(string) => match parser_tailwind::transpile_string(string) {
                    Ok(css) => self.css = css,
                    Err(error) => {
                        errors.push(CodeError {
                            error_type: Some(Box::new("SyntaxError".to_string())),
                            error_message: error.to_string(),
                            ..Default::default()
                        });
                    }
                },
                _ => errors.push(CodeError {
                    error_type: Some(Box::new("TypeError".to_string())),
                    error_message: format!(
                        "Expected expression to evaluate to a string, got a `{}` instead",
                        node.as_ref()
                    ),
                    ..Default::default()
                }),
            }
        }

        // Update errors
        self.errors = if errors.is_empty() {
            None
        } else {
            Some(errors)
        };

        Ok(())
    }
}