use crate::parser;
use pyo3::prelude::*;
use pyo3::types::PyDict;

type ActionEffectParams = (u8, u8, u8, u8, u16, u8, u8);
type StatusEffectParams = (u16, u16, f32, u32);

type StatuslistParams<'a> = (
    (u32, &'a str),
    &'a str,
    Vec<u32>,
    Vec<f32>,
    Vec<(u16, u16, f32, u32)>,
);

/// Get id name type from tuple
#[pyfunction]
#[pyo3(text_signature = "(name_id_pair: list[str]) -> (int, str)")]
pub(crate) fn parse_id_name_pair(inp: Vec<&str>) -> (u32, &str) {
    (
        parser::u32_from_param(inp.first().unwrap()),
        inp.last().unwrap(),
    )
}

/// Params to ability
#[pyfunction]
#[pyo3(text_signature = "(timestamp: int, params: list[str]) -> Ability")]
pub(crate) fn ability_from_params(timestamp: i64, inp: Vec<&str>) -> PyObject {
    let mut col = inp;
    let source_actor = col.drain(..2).collect::<Vec<&str>>();
    let ability = col.drain(..2).collect::<Vec<&str>>();
    let target_actor = col.drain(..2).collect::<Vec<&str>>();
    let action_effects = col.drain(..16).collect::<Vec<&str>>();
    let source_resources = col.drain(..6)
        .map(|x| parser::u32_from_str(x))
        .collect::<Vec<u32>>();
    let source_position = col.drain(..4)
        .map(|x| parser::f32_from_str(x))
        .collect::<Vec<f32>>();
    let target_resources = col.drain(..6)
        .map(|x| parser::u32_from_str(x))
        .collect::<Vec<u32>>();
    let target_position = col.drain(..4)
        .map(|x| parser::f32_from_str(x))
        .collect::<Vec<f32>>();
    let sequence = parser::u32_from_param(col.first().unwrap());
    Python::with_gil(|py| {
        let actor = PyModule::import(py, "nari.types.actor").unwrap()
            .getattr("Actor").unwrap();

        let source_actor = actor.call1(parse_id_name_pair(source_actor)).unwrap();
        let target_actor = actor.call1(parse_id_name_pair(target_actor)).unwrap();
        let source_resources_param = ((&source_resources[0]).to_owned(),(&source_resources[1]).to_owned(),(&source_resources[2]).to_owned(),(&source_resources[3]).to_owned(),(&source_resources[4]).to_owned(),(&source_resources[5]).to_owned());
        let target_resources_param = ((&target_resources[0]).to_owned(),(&target_resources[1]).to_owned(),(&target_resources[2]).to_owned(),(&target_resources[3]).to_owned(),(&target_resources[4]).to_owned(),(&target_resources[5]).to_owned());
        source_actor.getattr("resources").unwrap().call_method1("update", source_resources_param).unwrap();
        target_actor.getattr("resources").unwrap().call_method1("update", target_resources_param).unwrap();
        let source_position_param = ((&source_position[0]).to_owned(),(&source_position[1]).to_owned(),(&source_position[2]).to_owned(),(&source_position[3]).to_owned());
        let target_position_param = ((&target_position[0]).to_owned(),(&target_position[1]).to_owned(),(&target_position[2]).to_owned(),(&target_position[3]).to_owned());
        source_actor.getattr("position").unwrap().call_method1("update", source_position_param).unwrap();
        target_actor.getattr("position").unwrap().call_method1("update", target_position_param).unwrap();

        let ability_event = PyModule::import(py, "nari.types.ability").unwrap()
            .getattr("Ability").unwrap().call1(parse_id_name_pair(ability)).unwrap();

        let action_effect = PyModule::import(py, "nari.types.actioneffect").unwrap()
            .getattr("ActionEffect").unwrap();

        let action_effects = action_effects
            .chunks(2)
            .map(|x| {
                let (param0, param1, severity, effect_category, value, flags, multiplier) = action_effect_from_params(x.to_vec());
                let kwargs = PyDict::new(py);
                kwargs.set_item("effect_category", effect_category).unwrap();
                kwargs.set_item("severity", severity).unwrap();
                kwargs.set_item("flags", flags).unwrap();
                kwargs.set_item("value", value).unwrap();
                kwargs.set_item("multiplier", multiplier).unwrap();
                kwargs.set_item("additional_params", (param0, param1)).unwrap();
                action_effect.call((), Some(kwargs)).unwrap()
            }).collect::<Vec<&PyAny>>();

        let kwargs = PyDict::new(py);
        kwargs.set_item("timestamp", timestamp).unwrap();
        kwargs.set_item("action_effects", action_effects).unwrap();
        kwargs.set_item("source_actor", source_actor).unwrap();
        kwargs.set_item("target_actor", target_actor).unwrap();
        kwargs.set_item("ability", ability_event).unwrap();
        kwargs.set_item("sequence_id", sequence).unwrap();

        PyModule::import(py, "nari.types.event.ability").unwrap()
            .getattr("Ability").unwrap().call((), Some(kwargs)).unwrap().to_object(py)
    })
}

/// Params to action_effect
#[pyfunction]
#[pyo3(text_signature = "(params: list[str]) -> list[int]")]
pub(crate) fn action_effect_from_params(inp: Vec<&str>) -> ActionEffectParams {
    let mut num = parser::u32_from_param(inp[0]);
    let param0 = (&num >> 24) as u8;
    let param1 = (&num >> 16) as u8;
    let severity = (&num >> 8) as u8;
    let effect_category = num as u8;
    num = parser::u32_from_param(inp[1]);
    let value = (&num >> 16) as u16;
    let flags = (&num >> 8) as u8;
    let multiplier = num as u8;
    (param0, param1, severity, effect_category, value, flags, multiplier)
}

/// Params to status_effect
#[pyfunction]
#[pyo3(text_signature = "(params: list[str]) -> (int, int, float, int")]
pub(crate) fn status_effect_from_params(inp: Vec<&str>) -> StatusEffectParams {
    let (param0, param1) = parser::u16x2_from_param(inp.get(0).unwrap());
    (
        param0,
        param1,
        parser::f32_from_param(inp.get(1).unwrap()),
        parser::u32_from_param(inp.get(2).unwrap()),
    )
}

/// Params to statuslist
#[pyfunction]
#[pyo3(text_signature = "(params: list[str]) -> list[any]")]
pub(crate) fn statuslist_from_params(inp: Vec<&str>) -> StatuslistParams {
    let mut col = inp;
    let actor = col.drain(..2).collect::<Vec<&str>>();
    let class = col.drain(..1).collect::<Vec<&str>>();
    let resources = col.drain(..6).collect::<Vec<&str>>();
    let position = col.drain(..4).collect::<Vec<&str>>();
    let status_effects = col.drain(..col.len() - 1).collect::<Vec<&str>>();
    (
        parse_id_name_pair(actor),
        class.first().unwrap(),
        resources.iter().map(|x| parser::u32_from_param(x)).collect(),
        position.iter().map(|x| parser::f32_from_param(x)).collect(),
        status_effects
            .chunks(3)
            .map(|x| status_effect_from_params(x.to_vec()))
            .collect(),
    )
}
