use tea_codec::define_scope;

define_scope! {
	WasccCodec {
		GeneralWascc;
		BadDispatch;
		GeneralHost;
		NoSuchFunction;
		WasmMisc;
		HostCallFailure;
		GuestCallFailure;
		WapcGeneral;
		WascapGeneral;
		HostAuthorization;
		CapabilityProvider;
		MiscHost;
		Plugin;
		Middleware;
		ActorToActorCallNotExist;
		Invocation;
		KeyValue;
		Messaging;
		EnvVar;
		DiscardMessage;
		ProviderOperationRejected;
	}
}
