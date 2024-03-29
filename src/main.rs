use bevy::{ecs::event::Events, prelude::*};
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsolePlugin, PrintConsoleLine};
use bevy_mod_scripting::prelude::*;
use bevy_script_api::common::bevy::ScriptWorld;
use clap::Parser;

use bevy::{
    app::App, window::{PrimaryWindow, WindowResized}
};

#[derive(Clone)]
pub struct MyRhaiArgStruct {
    // ...
}

use bevy_mod_scripting_rhai::{
    assets::RhaiFile, rhai::{self, FuncArgs}, RhaiContext, RhaiEvent, RhaiScriptHost
};
//use bevy_mod_scripting_rhai::rhai::packages::Package;
//use bevy_script_api::rhai::{std::RegisterVecType, RegisterForeignRhaiType};


impl FuncArgs for MyRhaiArgStruct {
    fn parse<ARGS: Extend<rhai::Dynamic>>(self, _args: &mut ARGS) {
        // ...
    }
}

pub fn trigger_on_update_rhai(mut w: PriorityEventWriter<RhaiEvent<()>>) {
    let event = RhaiEvent {
        hook_name: "on_update".to_string(),
        args: (),
        recipients: Recipients::All,
    };

    w.send(event, 0);
}

pub fn forward_script_err_to_console(
    mut r: EventReader<ScriptErrorEvent>,
    mut w: EventWriter<PrintConsoleLine>,
) {
    for e in r.read() {
        w.send(PrintConsoleLine {
            line: format!("ERROR:{}", e.error).into(),
        });
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "run_script")]
pub struct RunScriptCmd {
    /// the relative path to the script, e.g.: `/hello.lua` for a script located in `assets/scripts/hello.lua`
    pub path: String,

    /// the entity id to attach this script to
    pub entity: Option<u32>,
}

pub fn run_script_cmd(
    mut log: ConsoleCommand<RunScriptCmd>,
    server: Res<AssetServer>,
    mut commands: Commands,
    mut existing_scripts: Query<&mut ScriptCollection<RhaiFile>>,
) {
    if let Some(Ok(RunScriptCmd { path, entity })) = log.take() {
        let handle = server.load::<RhaiFile>(&format!("scripts/{}", &path));

        match entity {
            Some(e) => {
                if let Ok(mut scripts) = existing_scripts.get_mut(Entity::from_raw(e)) {
                    info!("Creating script: scripts/{} {:?}", &path, e);

                    scripts.scripts.push(Script::<RhaiFile>::new(path, handle));
                } else {
                    log.reply_failed("Something went wrong".to_string());
                };
            }
            None => {
                info!("Creating script: scripts/{}", &path);

                commands.spawn(()).insert(ScriptCollection::<RhaiFile> {
                    scripts: vec![Script::<RhaiFile>::new(path, handle)],
                });
            }
        };
    }
}

// pub fn load_a_script(
//     server: Res<AssetServer>,
//     mut commands: Commands,
// ) {
//     // this handle is kept by the script so it will not be unloaded
//     let path = "scripts/console_integration.lua".to_string();
//     let handle = server.load::<LuaFile>(&path);

//     commands.spawn(()).insert(ScriptCollection::<LuaFile> {
//         scripts: vec![Script::<LuaFile>::new(
//             path, handle,
//         )],
//     });
// }

fn main() -> std::io::Result<()> {
    //let runscript_system = IntoSystem::into_system(run_script_cmd);

    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(ScriptingPlugin)
        .add_plugins(ConsolePlugin)
        // register bevy_console commands
        .add_console_command::<RunScriptCmd, _>(run_script_cmd)
        //.add_console_command::<DeleteScriptCmd, _>(delete_script_cmd)
        // choose and register the script hosts you want to use
        .add_script_host::<RhaiScriptHost<()>>(PostUpdate)
        // .add_api_provider::<RhaiScriptHost<()>>(Box::new(RhaiAPI))
        // .add_api_provider::<RhaiScriptHost<()>>(Box::new(RhaiBevyAPIProvider))
        .add_script_handler::<RhaiScriptHost<()>, 0, 0>(PostUpdate)
        // add your systems
        .add_systems(Update, trigger_on_update_rhai)
        .add_systems(Update, forward_script_err_to_console);
    // generate events for scripts to pickup
    //.add_systems(Update, trigger_on_update_rhai)

    // attach script components to entities
        
    info!("press '~' to open the console. Type in `run_script \"console_integration.rhai\"` to run example script!");

    app.run();

    Ok(())
}