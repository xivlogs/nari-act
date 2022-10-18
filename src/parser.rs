use crate::number_parser;
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

/// Params to ability
#[pyfunction]
#[pyo3(text_signature = "(timestamp: int, params: list[str]) -> Ability")]
pub(crate) fn ability_from_params(timestamp: i64, inp: Vec<&str>) -> PyObject {
    let mut col = inp;
    let source_actor = col.drain(..2).collect::<Vec<&str>>();
    let ability = col.drain(..2).collect::<Vec<&str>>();
    let target_actor = col.drain(..2).collect::<Vec<&str>>();
    let action_effects = col.drain(..16).collect::<Vec<&str>>()
        .chunks(2)
        .map(|x| action_effect_from_params(x.to_vec()))
        .collect::<Vec<ActionEffectParams>>();
    let source_resources = col.drain(..6)
        .map(|x| number_parser::u32_from_str(x))
        .collect::<Vec<u32>>();
    let source_position = col.drain(..4)
        .map(|x| number_parser::f32_from_str(x))
        .collect::<Vec<f32>>();
    let target_resources = col.drain(..6)
        .map(|x| number_parser::u32_from_str(x))
        .collect::<Vec<u32>>();
    let target_position = col.drain(..4)
        .map(|x| number_parser::f32_from_str(x))
        .collect::<Vec<f32>>();
    let sequence = number_parser::u32_from_param(col.first().unwrap());
    Python::with_gil(|py| {
        let source_actor_obj = create_id_name_pair(py, "nari.types.actor", "Actor", source_actor);
        let target_actor_obj = create_id_name_pair(py, "nari.types.actor", "Actor", target_actor);
        update_position(source_actor_obj, source_position);
        update_position(target_actor_obj, target_position);
        update_resources(source_actor_obj, source_resources);
        update_resources(target_actor_obj, target_resources);

        let ability_event = create_id_name_pair(py, "nari.types.ability", "Ability", ability);

        let action_effect = PyModule::import(py, "nari.types.actioneffect").unwrap()
            .getattr("ActionEffect").unwrap();

        let action_effects = action_effects
            .iter()
            .map(|x| {
                let (param0, param1, severity, effect_category, value, flags, multiplier) = x;
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
        kwargs.set_item("source_actor", source_actor_obj).unwrap();
        kwargs.set_item("target_actor", target_actor_obj).unwrap();
        kwargs.set_item("ability", ability_event).unwrap();
        kwargs.set_item("sequence_id", sequence).unwrap();

        PyModule::import(py, "nari.types.event.ability").unwrap()
            .getattr("Ability").unwrap().call((), Some(kwargs)).unwrap().to_object(py)
    })
}

/// Params to statuslist
#[pyfunction]
#[pyo3(text_signature = "(params: list[str]) -> tuple[tuple[int, str], str, list[int], list[int], list[tuple[int, int, int, int]]]")]
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
        resources.iter().map(|x| number_parser::u32_from_param(x)).collect(),
        position.iter().map(|x| number_parser::f32_from_param(x)).collect(),
        status_effects
            .chunks(3)
            .map(|x| status_effect_from_params(x.to_vec()))
            .collect(),
    )
}

/// Instantiates a new type of IdNamePair from nari.types requires an instance of Python::with_gil
///
/// Example: create_id_name_pair(py, "nari.types.actor", "Actor", vec![id, name])
fn create_id_name_pair<'a>(py: Python<'a>, module: &'a str, class: &'a str, id_name_pair: Vec<&'a str>) -> &'a PyAny {
    PyModule::import(py, module).unwrap()
        .getattr(class).unwrap()
        .call1(parse_id_name_pair(id_name_pair)).unwrap()
}

/// Updates nari.types.actor.Actor position values
fn update_position(actor: &PyAny, position: Vec<f32>) {
    actor.getattr("position").unwrap()
        .call_method1("update", (
            (&position[0]).to_owned(),
            (&position[1]).to_owned(),
            (&position[2]).to_owned(),
            (&position[3]).to_owned())
        ).unwrap();
}

/// Updates nari.types.actor.Actor resources values
fn update_resources(actor: &PyAny, resources: Vec<u32>) {
    actor.getattr("resources").unwrap()
        .call_method1("update", (
            (&resources[0]).to_owned(),
            (&resources[1]).to_owned(),
            (&resources[2]).to_owned(),
            (&resources[3]).to_owned(),
            (&resources[4]).to_owned(),
            (&resources[5]).to_owned())
        ).unwrap();
}

/// Get id name type from tuple
fn parse_id_name_pair(inp: Vec<&str>) -> (u32, &str) {
    (
        number_parser::u32_from_param(inp.first().unwrap()),
        inp.last().unwrap(),
    )
}

/// Params to action_effect
fn action_effect_from_params(inp: Vec<&str>) -> ActionEffectParams {
    let mut num = number_parser::u32_from_param(inp[0]);
    let param0 = (&num >> 24) as u8;
    let param1 = (&num >> 16) as u8;
    let severity = (&num >> 8) as u8;
    let effect_category = num as u8;
    num = number_parser::u32_from_param(inp[1]);
    let value = (&num >> 16) as u16;
    let flags = (&num >> 8) as u8;
    let multiplier = num as u8;
    (param0, param1, severity, effect_category, value, flags, multiplier)
}

/// Params to status_effect
fn status_effect_from_params(inp: Vec<&str>) -> StatusEffectParams {
    let (param0, param1) = number_parser::u16x2_from_param(inp.get(0).unwrap());
    (
        param0,
        param1,
        number_parser::f32_from_param(inp.get(1).unwrap()),
        number_parser::u32_from_param(inp.get(2).unwrap()),
    )
}
