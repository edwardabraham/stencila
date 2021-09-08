#![recursion_limit = "256"]

use neon::prelude::*;

mod client;
mod config;
mod documents;
mod errors;
mod formats;
mod graphs;
mod logging;
mod plugins;
mod prelude;
mod projects;
mod pubsub;
mod sessions;
mod sources;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("documentsSchemas", documents::schemas)?;
    cx.export_function("documentsList", documents::list)?;
    cx.export_function("documentsCreate", documents::create)?;
    cx.export_function("documentsOpen", documents::open)?;
    cx.export_function("documentsGet", documents::get)?;
    cx.export_function("documentsAlter", documents::alter)?;
    cx.export_function("documentsRead", documents::read)?;
    cx.export_function("documentsWrite", documents::write)?;
    cx.export_function("documentsWriteAs", documents::write_as)?;
    cx.export_function("documentsDump", documents::dump)?;
    cx.export_function("documentsLoad", documents::load)?;
    cx.export_function("documentsSubscribe", documents::subscribe)?;
    cx.export_function("documentsUnsubscribe", documents::unsubscribe)?;
    cx.export_function("documentsClose", documents::close)?;

    cx.export_function("projectsSchemas", projects::schemas)?;
    cx.export_function("projectsList", projects::list)?;
    cx.export_function("projectsOpen", projects::open)?;
    cx.export_function("projectsWrite", projects::write)?;
    cx.export_function("projectsClose", projects::close)?;
    cx.export_function("projectsAddSource", projects::add_source)?;
    cx.export_function("projectsRemoveSource", projects::remove_source)?;
    cx.export_function("projectsImportSource", projects::import_source)?;
    cx.export_function("projectsGraph", projects::graph)?;

    cx.export_function("formatsSchemas", formats::schemas)?;
    cx.export_function("formatsFormats", formats::formats)?;

    cx.export_function("graphsSchemas", graphs::schemas)?;

    cx.export_function("sourcesSchemas", sources::schemas)?;

    cx.export_function("sessionsSchemas", sessions::schemas)?;

    cx.export_function("pluginsSchema", plugins::schema)?;
    cx.export_function("pluginsList", plugins::list)?;
    cx.export_function("pluginsInstall", plugins::install)?;
    cx.export_function("pluginsUninstall", plugins::uninstall)?;
    cx.export_function("pluginsUpgrade", plugins::upgrade)?;
    cx.export_function("pluginsRefresh", plugins::refresh)?;

    cx.export_function("configSchemas", config::schema)?;
    cx.export_function("configGet", config::get)?;
    cx.export_function("configSet", config::set)?;
    cx.export_function("configValidate", config::validate)?;
    cx.export_function("configSetProperty", config::set_property)?;
    cx.export_function("configResetProperty", config::reset_property)?;

    cx.export_function("pubsubInit", pubsub::init)?;
    cx.export_function("pubsubSubscribe", pubsub::subscribe)?;
    cx.export_function("pubsubUnsubscribe", pubsub::unsubscribe)?;
    cx.export_function("pubsubPublish", pubsub::publish)?;

    cx.export_function("loggingInit", logging::init)?;
    cx.export_function("loggingTest", logging::test)?;

    cx.export_function("errorsSchema", errors::schema)?;
    cx.export_function("errorsStart", errors::start)?;
    cx.export_function("errorsStop", errors::stop)?;

    Ok(())
}
