pub struct Break;

pub type Continue = ();

pub struct Kill<T = ()>(pub T);

pub struct ActorCommand<ShutDown>(pub Result<Continue, Result<Break, ShutDown>>);

impl<ShutDown> From<()> for ActorCommand<ShutDown> {
    #[inline(always)]
    fn from(_: ()) -> Self {
        ActorCommand(Ok(()))
    }
}


impl<ShutDown> From<Break> for ActorCommand<ShutDown> {
    #[inline(always)]
    fn from(_: Break) -> Self {
        ActorCommand(Err(Ok(Break)))
    }
}


impl<ShutDown, K> From<Kill<K>> for ActorCommand<ShutDown>
where
    ShutDown: From<K>,
{
    #[inline(always)]
    fn from(Kill(k): Kill<K>) -> Self {
        let shutdown: ShutDown = k.into();
        ActorCommand(Err(Err(shutdown)))
    }
}

impl<ShutDown, Cmd> From<Option<Cmd>> for ActorCommand<ShutDown>
where
    Cmd: Into<ActorCommand<ShutDown>>,
{
    #[inline(always)]
    fn from(opt: Option<Cmd>) -> Self {
        if let Some(command) = opt {
            command.into()
        } else {
            ActorCommand(Ok(()))
        }
    }
}

impl<ShutDown, Cmd1, Cmd2> From<Result<Cmd1, Cmd2>> for ActorCommand<ShutDown>
where
    Cmd1: Into<ActorCommand<ShutDown>>,
    Cmd2: Into<ActorCommand<ShutDown>>,
{
    #[inline(always)]
    fn from(resp: Result<Cmd1, Cmd2>) -> Self {
        match resp {
            Ok(command) => command.into(),
            Err(command) => command.into(),
        }
    }
}
