pub struct Break;

pub type Continue = ();

pub struct Cmd<T = ()>(pub T);

pub struct ActorCommand<Command>(pub Result<Continue, Result<Break, Command>>);

impl<Command> From<()> for ActorCommand<Command> {
    #[inline(always)]
    fn from(_: ()) -> Self {
        ActorCommand(Ok(()))
    }
}

impl<Command> From<Break> for ActorCommand<Command> {
    #[inline(always)]
    fn from(_: Break) -> Self {
        ActorCommand(Err(Ok(Break)))
    }
}

impl<Command, C> From<Cmd<C>> for ActorCommand<Command>
where
    Command: From<C>,
{
    #[inline(always)]
    fn from(Cmd(k): Cmd<C>) -> Self {
        let shutdown: Command = k.into();
        ActorCommand(Err(Err(shutdown)))
    }
}

impl<Command, C> From<Option<C>> for ActorCommand<Command>
where
    C: Into<ActorCommand<Command>>,
{
    #[inline(always)]
    fn from(opt: Option<C>) -> Self {
        if let Some(command) = opt {
            command.into()
        } else {
            ActorCommand(Ok(()))
        }
    }
}

impl<Command, C1, C2> From<Result<C1, C2>> for ActorCommand<Command>
where
    C1: Into<ActorCommand<Command>>,
    C2: Into<ActorCommand<Command>>,
{
    #[inline(always)]
    fn from(resp: Result<C1, C2>) -> Self {
        match resp {
            Ok(command) => command.into(),
            Err(command) => command.into(),
        }
    }
}
