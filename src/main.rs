use bevy::{
    app::App, prelude::*, window::{PrimaryWindow, WindowResized}
};
use bevy_console::{
    clap::Parser, AddConsoleCommand, ConsoleCommand, ConsolePlugin, PrintConsoleLine, ConsoleCommandEntered
};

#[derive(Clone)]
pub struct MyRhaiArgStruct {
    // ...
}

use bevy_mod_scripting::prelude::*;
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

// pub fn forward_script_err_to_console(
//     mut r: EventReader<ScriptErrorEvent>,
//     mut w: EventWriter<PrintConsoleLine>,
// ) {
//     for e in r.read() {
//         w.send(PrintConsoleLine {
//             line: format!("ERROR:{}", e.error).into(),
//         });
//     }
// }

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
    let path = "run.rhai".to_string();
    let entity = Some(1u32); // replace 1 with your desired entity id

    //if let Some(Ok(RunScriptCmd { path, entity })) = log.take() {
    if true {
        let handle = server.load::<RhaiFile>(&format!("scripts/{}", &path));    //if let Some(Ok(RunScriptCmd { path, entity })) = log.take() {
        info!("[run_script_cmd] Processing script: scripts/{}", &path);

        // match entity {
        //     Some(e) => {
        //         if let Ok(mut scripts) = existing_scripts.get_mut(Entity::from_raw(e)) {
        //             info!("Creating script: scripts/{} {:?}", &path, e);

        //             scripts.scripts.push(Script::<RhaiFile>::new(path, handle));
        //         } else {
        //             log.reply_failed("Something went wrong".to_string());
        //         };
        //     }
        //     None => {
        info!("Creating script: scripts/{}", &path);

        commands.spawn(()).insert(ScriptCollection::<RhaiFile> {
            scripts: vec![Script::<RhaiFile>::new(path, handle)],
        });
        //     }
        // };
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
        app.add_plugins(ScriptingPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(ConsolePlugin)
        .add_console_command::<RunScriptCmd, _>(run_script_cmd)
        // pick and register only the hosts you want to use
        // use any system set AFTER any systems which add/remove/modify script components
        // in order for your script updates to propagate in a single frame
        .add_script_host::<RhaiScriptHost<MyRhaiArgStruct>>(PostUpdate)
        .add_systems(Startup, run_script_cmd)
        // the handlers should be ran after any systems which produce script events.
        // The PostUpdate set is okay only if your API doesn't require the core Bevy systems' commands
        // to run beforehand.
        // Note, this setup assumes a single script handler system set with all events having identical
        // priority of zero (see examples for more complex scenarios)
        .add_script_handler::<RhaiScriptHost<MyRhaiArgStruct>, 0, 0>(
             PostUpdate,
        );

        // generate events for scripts to pickup
        //.add_systems(Update, trigger_on_update_rhai)

        // attach script components to entities
        
    app.run();

    Ok(())
}