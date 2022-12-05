const s1 = 1;
const s2 = 2;
const n1 = 1;
const el = 1;
const $cmpt = 1;

const c1 = (
  <div ref={el} class={$cmpt}>
    <Comp prop1={s1}></Comp>
    <div1>''""</div1>
    <div2>{s1}</div2>
    <div3 ref={n1} class={"cls1"}></div3>
    <div4 class={s1}></div4>
    <div4 s:class={s2}></div4>
    <div4 style={s1}></div4>
    <div4 s:style={s1}></div4>
    <div4 id={n1}></div4>
    <div4 s:id={n1}></div4>
    <div6>
      <div7></div7>
      <div8 s:class={s2}></div8>
    </div6>
    <div5 on:click={() => console.log(el)}></div5>
    <div5 on:click={[() => console.log(1), {}]}></div5>
  </div>
);
