fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .enum_attribute(
            "vorpal.artifact.v0.ArtifactSystem",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .message_attribute(
            "vorpal.config.v0.Config",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .message_attribute(
            "vorpal.artifact.v0.ArtifactId",
            "#[derive(Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]",
        )
        .message_attribute(
            "vorpal.artifact.v0.ArtifactSourceId",
            "#[derive(Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]",
        )
        .message_attribute(
            "vorpal.artifact.v0.ArtifactSource",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .message_attribute(
            "vorpal.artifact.v0.ArtifactStepEnvironment",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .message_attribute(
            "vorpal.artifact.v0.ArtifactStep",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .message_attribute(
            "vorpal.artifact.v0.Artifact",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .message_attribute(
            "vorpal.artifact.v0.ArtifactBuildRequest",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .compile_protos(
            &[
                "v0/artifact/artifact.proto",
                "v0/config/config.proto",
                "v0/registry/registry.proto",
            ],
            &["api"],
        )?;
    Ok(())
}
