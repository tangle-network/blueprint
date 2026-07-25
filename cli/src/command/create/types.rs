use clap::Args;

#[derive(Debug, Clone, Args)]
pub struct CreateArgs {
    /// The name of the blueprint
    #[arg(short, long, value_name = "NAME", env = "NAME")]
    pub name: String,

    #[command(flatten)]
    pub blueprint_type: BlueprintType,
}

#[derive(Debug, Clone, Args, Default)]
#[group(required = false, multiple = false)]
pub struct BlueprintType {
    /// Create a Tangle blueprint
    #[arg(long)]
    pub tangle: bool,
}

impl BlueprintType {
    pub fn get_type(&self) -> Option<BlueprintVariant> {
        if self.tangle {
            Some(BlueprintVariant::Tangle)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum BlueprintVariant {
    Tangle,
}
