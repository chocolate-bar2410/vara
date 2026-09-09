export type UIInstance = {
    run : (self: UIInstance) -> (),
    get : <T>(self: UIInstance,property : string) -> T,
    set : <T>(self: UIInstance,property : string,value : T) -> (),
    connect : (self: UIInstance,(...unknown) -> unknown) -> (),
}

declare io : {
    write : (message : string) -> (),
    read : () -> string,
    readline : () -> string,
}

declare fs : {
    write : () -> (),
    read : (path : string,contents : buffer | string) -> string,
}

declare gui : {
    registerFile: (path: string) -> (),
    createInstance : (component: string) -> UIInstance,
}