pub struct Break;

pub type Continue = ();

pub struct Cmd<T = ()>(pub T);

pub struct ActorCmdRes<C>(pub Result<Continue, Result<Break, C>>);

impl<C> From<()> for ActorCmdRes<C> {
    #[inline(always)]
    fn from(_: ()) -> Self {
        ActorCmdRes(Ok(()))
    }
}

impl<C> From<Break> for ActorCmdRes<C> {
    #[inline(always)]
    fn from(_: Break) -> Self {
        ActorCmdRes(Err(Ok(Break)))
    }
}

impl<C, K> From<Cmd<K>> for ActorCmdRes<C>
where
    C: From<K>,
{
    #[inline(always)]
    fn from(Cmd(k): Cmd<K>) -> Self {
        let cmd: C = k.into();
        ActorCmdRes(Err(Err(cmd)))
    }
}

impl<C, InnerCmd> From<Option<InnerCmd>> for ActorCmdRes<C>
where
    InnerCmd: Into<ActorCmdRes<C>>,
{
    #[inline(always)]
    fn from(opt: Option<InnerCmd>) -> Self {
        if let Some(command) = opt {
            command.into()
        } else {
            ActorCmdRes(Ok(()))
        }
    }
}

impl<C, InnerCmd1, InnerCmd2> From<Result<InnerCmd1, InnerCmd2>> for ActorCmdRes<C>
where
    InnerCmd1: Into<ActorCmdRes<C>>,
    InnerCmd2: Into<ActorCmdRes<C>>,
{
    #[inline(always)]
    fn from(resp: Result<InnerCmd1, InnerCmd2>) -> Self {
        match resp {
            Ok(command) => command.into(),
            Err(command) => command.into(),
        }
    }
}
