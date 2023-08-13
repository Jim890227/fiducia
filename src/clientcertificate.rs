use clap::
{
    Args,
    Parser,
    Subcommand
};

#[derive(Parser, Debug)]
#[command(author, version, about)]

pub struct ClientCertificate
{
    #[clap(subcommand)]
    pub command: EntityType,
}

impl ClientCertificate
{
    pub fn get(&self) -> (&str,&str,&str,&str)
    {
        self.command.get()
    }
}

#[derive(Debug,Subcommand)]

pub enum EntityType
{
    cert(ClientCreate),
}
impl EntityType
{
    pub fn get(&self) -> (&str,&str,&str,&str)
    {
        match self
        {
            Self::cert(args) => args.command.get()
        }
    }
}

#[derive(Debug,Args)]
pub struct ClientCreate
{
    #[clap(subcommand)]
    pub command: ClientCommand,
}

#[derive(Debug,Subcommand)]
pub enum ClientCommand
{
    client(Client),
}
impl ClientCommand
{
    pub fn get(&self) -> (&str,&str,&str,&str)
    {
        match self
        {
            Self::client(args) => args.command.get()
        }
    }
}

#[derive(Debug,Args)]
pub struct Client
{
    #[clap(subcommand)]
    pub command: CreateCommand,
}

#[derive(Debug,Subcommand)]
pub enum CreateCommand
{
    create(CreateCertificate),
}
impl CreateCommand
{
    pub fn get(&self) -> (&str,&str,&str,&str)
    {
        match self
        {
            Self::create(args) => (&args.profile, &args.common_name, &args.duration, &args.key_size),
        }
    }
}

#[derive(Debug,Args)]
pub struct CreateCertificate
{
    #[arg(long)]
    pub profile: String,
    #[arg(long)]
    pub common_name: String,
    #[arg(long)]
    pub duration: String,
    #[arg(long)]
    pub key_size: String,

}
