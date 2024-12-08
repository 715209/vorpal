use crate::config::{artifact::add_artifact, ContextConfig};
use anyhow::{bail, Result};
use indoc::formatdoc;
use vorpal_schema::vorpal::artifact::v0::{
    ArtifactId,
    ArtifactSystem::{Aarch64Linux, Aarch64Macos, UnknownSystem, X8664Linux, X8664Macos},
};

pub fn artifact(context: &mut ContextConfig) -> Result<ArtifactId> {
    let name = "protoc";

    let systems = vec![
        Aarch64Linux.into(),
        Aarch64Macos.into(),
        X8664Linux.into(),
        X8664Macos.into(),
    ];

    add_artifact(
        context,
        vec![],
        vec![],
        name,
        formatdoc! {"
            curl -L -o ./protoc-{version}-{target}.zip \
                https://github.com/protocolbuffers/protobuf/releases/download/v{version}/protoc-{version}-{target}.zip

            unzip ./protoc-{version}-{target}.zip -d $VORPAL_OUTPUT

            chmod +x \"$VORPAL_OUTPUT/bin/protoc\"",
            target = match context.get_target() {
                Aarch64Linux => "linux-aarch_64",
                Aarch64Macos => "osx-aarch_64",
                X8664Linux => "linux-x86_64",
                X8664Macos => "osx-x86_64",
                UnknownSystem => bail!("Unsupported system: {:?}", context.get_target()),
            },
            version = "25.4",
        },
        vec![],
        systems,
    )
}
