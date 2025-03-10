use std::collections::{HashMap, HashSet};

use idesyde_blueprints::execute_standalone_identification_module;
use idesyde_core::{
    headers::{self, DesignModelHeader},
    DecisionModel, DesignModel, MarkedIdentificationRule, StandaloneIdentificationModule,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulinkReactiveDesignModel {
    pub processes: HashSet<String>,
    pub processes_sizes: HashMap<String, u32>,
    pub delays: HashSet<String>,
    pub delays_sizes: HashMap<String, u32>,
    pub sources: HashSet<String>,
    pub sources_sizes: HashMap<String, u32>,
    pub sources_periods_numen: HashMap<String, i32>,
    pub sources_periods_denom: HashMap<String, i32>,
    pub constants: HashSet<String>,
    pub sinks: HashSet<String>,
    pub sinks_sizes: HashMap<String, u32>,
    pub sinks_deadlines_numen: HashMap<String, i32>,
    pub sinks_deadlines_denom: HashMap<String, i32>,
    pub processes_operations: HashMap<String, HashMap<String, HashMap<String, u32>>>,
    pub delays_operations: HashMap<String, HashMap<String, HashMap<String, u32>>>,
    pub links_src: Vec<String>,
    pub links_dst: Vec<String>,
    pub links_src_port: Vec<String>,
    pub links_dst_port: Vec<String>,
    pub links_size: Vec<u32>,
}

impl DesignModel for SimulinkReactiveDesignModel {
    fn unique_identifier(&self) -> String {
        "SimulinkReactiveDesignModel".to_string()
    }

    fn header(&self) -> headers::DesignModelHeader {
        let mut elems: HashSet<String> = HashSet::new();
        elems.extend(self.processes.iter().map(|x| x.to_owned()));
        elems.extend(self.delays.iter().map(|x| x.to_owned()));
        elems.extend(self.sources.iter().map(|x| x.to_owned()));
        elems.extend(self.constants.iter().map(|x| x.to_owned()));
        elems.extend(self.sinks.iter().map(|x| x.to_owned()));
        for i in 0..self.links_src.len() {
            elems.insert(format!(
                "{}={}:{}-{}:{}",
                self.links_size[i],
                self.links_src[i],
                self.links_src_port[i],
                self.links_dst[i],
                self.links_dst_port[i]
            ));
        }
        DesignModelHeader {
            category: self.unique_identifier(),
            model_paths: Vec::new(),
            elements: elems.into_iter().collect(),
        }
    }
}

fn partially_identify_wokload_model(
    design_models: &Vec<Box<dyn DesignModel>>,
    _decision_models: &Vec<Box<dyn DecisionModel>>,
) -> Vec<Box<dyn DecisionModel>> {
    let mut _identified = Vec::new();
    // let mut procs: HashSet<String> = HashSet::new();
    // let mut delays: HashSet<String> = HashSet::new();
    // let mut linksWithoutConstants: HashMap<String, HashMap<String, u32>> = HashMap::new();
    // let mut allLinks: HashMap<String, HashMap<String, u32>> = HashMap::new();
    for design_model in design_models {
        if design_model.unique_identifier() == "SimulinkReactiveDesignModel" {}
    }
    _identified
}
// struct MatlabIdentificationModule {}

// impl StandaloneIdentificationModule for MatlabIdentificationModule {
//     fn uid(&self) -> String {
//         "MatlabIdentificationModule".to_string()
//     }

fn read_design_model(path: &std::path::Path) -> Option<Box<dyn idesyde_core::DesignModel>> {
    match path.extension().and_then(|x| x.to_str()) {
        Some("json") => {
            if path
                .file_name()
                .and_then(|x| x.to_str())
                .map(|x| !x.starts_with("header"))
                .unwrap_or(false)
            {
                if let Ok(s) = std::fs::read_to_string(&path) {
                    let m: SimulinkReactiveDesignModel =
                        serde_json::from_str(s.as_str()).expect("something");
                    Some(Box::new(m) as Box<dyn DesignModel>)
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

fn write_design_model(
    _design_model: &Box<dyn idesyde_core::DesignModel>,
    _dest: &std::path::Path,
) -> bool {
    false
}

fn decision_header_to_model(
    _header: &headers::DecisionModelHeader,
) -> Option<Box<dyn idesyde_core::DecisionModel>> {
    None
}

//     fn identification_rules(&self) -> Vec<idesyde_core::MarkedIdentificationRule> {
//     }

//     fn reverse_identification_rules(&self) -> Vec<idesyde_core::ReverseIdentificationRule> {
//         Vec::new()
//     }
// }
fn main() {
    execute_standalone_identification_module(StandaloneIdentificationModule::new(
        "MatlabIdentificationModule",
        vec![MarkedIdentificationRule::DesignModelOnlyIdentificationRule(
            partially_identify_wokload_model,
        )],
        vec![],
        read_design_model,
        write_design_model,
        decision_header_to_model,
        HashSet::new(),
    ));
}
