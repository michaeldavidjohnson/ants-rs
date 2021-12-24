struct InstanceInput {
    [[location(1)]] model_matrix_0: vec4<f32>;
    [[location(2)]] model_matrix_1: vec4<f32>;
    [[location(3)]] model_matrix_2: vec4<f32>;
    [[location(4)]] model_matrix_3: vec4<f32>;
    [[location(5)]] colour: vec3<f32>;
};

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
};

[[stage(vertex)]]
fn vs_main(model: VertexInput, instance: InstanceInput) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    var out: VertexOutput;

    out.color = instance.colour;
    out.clip_position = model_matrix * vec4<f32>(model.position, 1.0);

    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}