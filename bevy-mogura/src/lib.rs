use crate::structure::*;
use bevy::prelude::*;
use mogura_io::prelude::*;

mod camera;
mod gui;
mod light;
mod structure;
mod trajectory;

pub mod prelude {
    pub use crate::MoguraPlugins;
}

#[derive(Default, Clone)]
pub struct MoguraPlugins {
    pub input_structure_file: Option<String>,
    pub input_trajectory_file: Option<String>,
}

impl Plugin for MoguraPlugins {
    fn build(&self, app: &mut App) {
        let mogura_state = MoguraState::new(
            self.input_structure_file.clone(),
            self.input_trajectory_file.clone(),
        );

        bevy::asset::load_internal_asset!(
            app,
            structure::SHADER_HANDLE,
            "line_material.wgsl",
            Shader::from_wgsl
        );

        let mogura_selections = if self.input_structure_file.is_some() {
            MoguraSelections::new(1)
        } else {
            MoguraSelections::new(0)
        };

        app.insert_resource(mogura_state)
            .insert_resource(mogura_selections)
            // .add_systems(Startup, dbg::setup_test)
            .add_plugins(camera::MoguraCameraPlugins)
            .add_plugins(gui::MoguraGuiPlugins)
            .add_plugins(light::MoguraLightPlugins)
            .add_plugins(structure::MoguraStructurePlugins)
            .add_plugins(trajectory::MoguraTrajectoryPlugins);
    }
}

#[derive(Debug, Clone)]
pub struct EachSelection {
    pub atom_selection: String,
    pub drawing_method: structure::DrawingMethod,
    pub selected_atoms: std::collections::HashSet<usize>,
    pub selected_bonds: std::collections::HashSet<(usize, usize)>,
    pub redraw: bool,
    pub delete: bool,
}

impl EachSelection {
    #[allow(clippy::borrowed_box)]
    pub fn apply_selection(
        &mut self,
        structure_data: &Box<dyn StructureData>,
    ) -> Result<(), String> {
        let selection = mogura_asl::parse_selection(&self.atom_selection)?;
        let (selected_atoms, selected_bonds) = {
            let atoms = structure_data.as_ref().atoms();
            let bonds = structure_data.as_ref().bonds_indirected();
            let (selected_atoms, selected_bonds) = selection.select_atoms_bonds(atoms, &bonds);
            (selected_atoms, selected_bonds)
        };
        self.selected_atoms = selected_atoms;
        self.selected_bonds = selected_bonds;
        Ok(())
    }
}

impl Default for EachSelection {
    fn default() -> Self {
        Self {
            atom_selection: "all".to_string(),
            drawing_method: structure::DrawingMethod::BallAndStick,
            selected_atoms: std::collections::HashSet::new(),
            selected_bonds: std::collections::HashSet::new(),
            redraw: true,
            delete: false,
        }
    }
}

#[derive(Default, Resource)]
pub struct MoguraSelections(Vec<EachSelection>);

impl MoguraSelections {
    pub fn new(init_num: usize) -> Self {
        Self(vec![EachSelection::default(); init_num])
    }
}

#[derive(Resource)]
pub struct MoguraState {
    pub structure_file: Option<String>,
    pub structure_data: Option<Box<dyn StructureData>>,
    pub trajectory_file: Option<String>,
    pub trajectory_data: Option<Box<dyn TrajectoryData>>,
    pub update_trajectory: bool,
    pub update_tmp_trajectory: bool,
    pub loop_trajectory: bool,
    pub current_frame_id: usize,
    pub init_look_at: bool,
    pub logs: Vec<String>,
    // pub selections: Vec<EachSelection>,
}

impl MoguraState {
    pub fn new(structure_file: Option<String>, trajectory_file: Option<String>) -> Self {
        let structure_data = structure_file
            .as_ref()
            .map(|file| match structure_loader(file) {
                Ok(data) => data,
                Err(e) => {
                    panic!("Failed to load structure file: {}", e);
                }
            });
        let trajectory_data = if let Some(ref str_file) = structure_file {
            trajectory_file
                .as_ref()
                .map(|file| match trajectory_loader(str_file, file) {
                    Ok(data) => data,
                    Err(e) => {
                        panic!("Failed to load trajectory file: {}", e);
                    }
                })
        } else {
            None
        };

        Self {
            structure_data,
            structure_file,
            trajectory_data,
            trajectory_file,
            update_trajectory: false,
            update_tmp_trajectory: false,
            loop_trajectory: false,
            current_frame_id: 0,
            init_look_at: true,
            logs: Vec::with_capacity(100),
            // selections: vec![EachSelection::default()],
        }
    }

    pub fn n_frame(&self) -> Option<usize> {
        self.trajectory_data.as_ref().map(|td| td.n_frame())
    }

    pub fn next_frame_id(&mut self) {
        let n_frame = if let Some(n_frame) = self.n_frame() {
            n_frame
        } else {
            return;
        };
        self.current_frame_id += 1;
        if self.current_frame_id >= n_frame {
            self.current_frame_id = 0;
            self.update_trajectory = false;
        } else {
            self.update_trajectory = true;
        }
    }

    pub fn loop_frame_id(&mut self) {
        let n_frame = if let Some(n_frame) = self.n_frame() {
            n_frame
        } else {
            return;
        };
        self.current_frame_id += 1;
        if self.current_frame_id >= n_frame {
            self.current_frame_id = 0;
        }
    }
}

#[allow(dead_code)]
mod dbg {
    use bevy::prelude::*;
    pub fn setup_test(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        // circular base
        commands.spawn((
            Mesh3d(meshes.add(Circle::new(4.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ));
        // cube
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(0.0, 0.5, 0.0),
        ));
    }
}
