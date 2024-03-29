use bevy::{
    app::App,
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    // render::{
    //     render_resource::{Extent3d, TextureDimension, TextureFormat},
    //     texture::ImageSampler,
    // },
    window::{PrimaryWindow, WindowResized},
};

#[derive(Clone)]
pub struct MyRhaiArgStruct {
    // ...
}

use bevy_mod_scripting::prelude::*;
use bevy_mod_scripting_rhai::{
    rhai::{self, FuncArgs},
    RhaiScriptHost,
    //MyRhaiArgStruct,
    //RhaiEventArgs,
    RhaiEvent
};
//use bevy_mod_scripting_rhai::rhai::packages::Package;
//use bevy_script_api::rhai::{std::RegisterVecType, RegisterForeignRhaiType};


impl FuncArgs for MyRhaiArgStruct {
    fn parse<ARGS: Extend<rhai::Dynamic>>(self, _args: &mut ARGS) {
        // ...
    }
}

// event callback generator for rhai
// rhai event arguments can be any rust type implementing FuncArgs
pub fn trigger_on_update_rhai() { //mut w: PriorityEventWriter<RhaiEvent<MyRhaiArgStruct>>) {
    // let event = RhaiEvent {
    //     hook_name: "on_update".to_string(),
    //     args: MyRhaiArgStruct {},
    //     recipients: Recipients::All
    // };

    // w.send(event,0);
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();
        app.add_plugins(ScriptingPlugin)
        .add_plugins(DefaultPlugins)
        // pick and register only the hosts you want to use
        // use any system set AFTER any systems which add/remove/modify script components
        // in order for your script updates to propagate in a single frame
        //.add_script_host::<RhaiScriptHost<MyRhaiArgStruct>>(PostUpdate)

        // the handlers should be ran after any systems which produce script events.
        // The PostUpdate set is okay only if your API doesn't require the core Bevy systems' commands
        // to run beforehand.
        // Note, this setup assumes a single script handler system set with all events having identical
        // priority of zero (see examples for more complex scenarios)
        // .add_script_handler::<RhaiScriptHost<RhaiEventArgs>, 0, 0>(
        //     PostUpdate,
        // )

        // generate events for scripts to pickup
        .add_systems(Update, trigger_on_update_rhai);

        // attach script components to entities
        //.add_startup_system(load_a_script);
    app.run();

    Ok(())
}