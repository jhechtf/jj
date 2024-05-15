// Copyright 2020-2023 The Jujutsu Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::cli_util::CommandHelper;
use crate::command_error::CommandError;
use crate::ui::Ui;
use isl_server::setup_server;

/// Start the ISL server that allows API based interactivity
#[derive(clap::Args, Clone, Debug)]
#[command()]
pub(crate) struct IslArgs {
    /// The port for the item to open up
    #[arg(default_value = "6075", short = 'p')]
    port: Option<u16>,

    #[arg(default_value = "127.0.0.1", short = 'h')]
    host: Option<String>,
}

pub fn cmd_isl(ui: &mut Ui, command: &CommandHelper, args: &IslArgs) -> Result<(), CommandError> {
    let port = args.port.unwrap_or(6075);
    // let host = args.host.unwrap();
    let server = setup_server();

    if let Some(mut formatter) = ui.status_formatter() {
        write!(formatter, "Start port on {port}\n")?;
        write!(formatter, "Peenr")?;
    }

    Ok(())
}
