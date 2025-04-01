use crate::trajectory_data::*;

#[cfg(feature = "groan_rs")]
use groan_rs::prelude::*;

#[derive(Clone, Debug)]
pub struct XtcData {
    frames: Vec<Frame>,
}

#[cfg(feature = "groan_rs")]
impl TrajectoryData for XtcData {
    fn frames(&self) -> &Vec<Frame> {
        &self.frames
    }

    fn load(topology_file: &str, trajectory_file: &str) -> Result<Self, anyhow::Error> {
        let mut topology = System::from_file(topology_file).map_err(anyhow::Error::msg)?;

        let trajectory = topology.xtc_iter(trajectory_file)?;

        let (frame_size, _) = trajectory.size_hint();
        let mut frames = Vec::with_capacity(frame_size);
        for (frame_id, frame) in trajectory.enumerate() {
            match frame {
                Ok(frame) => {
                    let frame_atoms = frame.get_atoms_copy();
                    let mut positions = Vec::with_capacity(frame_atoms.len());
                    for atom in frame_atoms {
                        positions.push([
                            // xtc unit is nm
                            atom.get_position().unwrap().x * 10.0,
                            atom.get_position().unwrap().y * 10.0,
                            atom.get_position().unwrap().z * 10.0,
                        ]);
                    }
                    frames.push(Frame::new(frame_id, positions));
                }
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            }
        }
        Ok(Self { frames })
    }
}
