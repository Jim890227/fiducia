use clap::
{
    Args,
    Parser,
    Subcommand
};

#[derive(Parser, Debug)]
#[command(author, version, about)]

pub struct Route
{
    #[clap(subcommand)]
    pub command: EntityType,
}

impl Route
{
    pub fn get(&self) -> (&str,&str,&str,&str,&str)
    {
        self.command.get()
    }
}
#[derive(Debug,Subcommand)]

pub enum EntityType
{
    
    route(RouteCreate),
    
}
impl EntityType
{
    pub fn get(&self) -> (&str,&str,&str,&str,&str)
    {
        match self
        {
            Self::route(args) => args.command.get()

        }
    }
}

#[derive(Debug,Args)]
pub struct RouteCreate
{
    #[clap(subcommand)]
    pub command: RouteCommand,
}

#[derive(Debug,Subcommand)]
pub enum RouteCommand
{
    create(CreateRoute),
}
impl RouteCommand
{
    pub fn get(&self) -> (&str,&str,&str,&str,&str)
    {
        match self
        {
            Self::create(args) => (&args.profile, &args.dns_name, &args.backend_service_name, &args.backend_service_port, &args.backend_service_scheme),
        }
    }
}

#[derive(Debug,Args)]
pub struct CreateRoute
{
    #[arg(long)]
    pub profile: String,
    #[arg(long)]
    pub dns_name: String,
    #[arg(long)]
    pub backend_service_name: String,
    #[arg(long)]
    pub backend_service_port: String,
    #[arg(long)]
    pub backend_service_scheme: String,
}

