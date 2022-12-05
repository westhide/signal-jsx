const t1 = <div>1</div>;

const t2 = <div>1</div>;

const t3 = <div>1</div>;

type Signal<T> = T;

function signal<T>(v: T) {
    return v;
}

const v1 = 1;
{
    const v11 = 1;
}

let v2;

let v3 = 1, v4 = signal(5);

function A1() {
    return null;
}

const c1 = <div><A1></A1></div>

function A2() {
    return null;
}

const c2 = <A1><A2></A2></A1>

const c3 = <div><div></div><A2 class={"cls1"}></A2></div>