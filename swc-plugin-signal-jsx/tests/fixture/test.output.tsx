import { component$, toSibling } from "@westhide/tai";
const s1 = 1;
const s2 = 2;
const n1 = 1;
const el = 1;
const $cmpt = 1;
const c1 = (()=>{
    const $cmpt1 = Comp({
        prop1: s1
    });
    return component$(`<div >${$cmpt1.html}<div1 >''""</div1><div2 > </div2><div3 class="cls1"></div3><div4 ></div4><div4 ></div4><div4 ></div4><div4 ></div4><div4 ></div4><div4 ></div4><div6 ><div7 ></div7><div8 ></div8></div6><div5 ></div5><div5 ></div5></div>`, ($el)=>{
        el.value = $el;
        $el.className = $cmpt;
        const $el1 = $el.firstChild;
        $cmpt1.patch($el1);
        const $el2 = toSibling($el1, 2);
        const $el3 = $el2.firstChild;
        s1.subscribe(()=>{
            $el3.textContent = s1.value;
        });
        const $el4 = $el2.nextSibling;
        n1.value = $el4;
        const $el5 = $el4.nextSibling;
        $el5.className = s1;
        const $el6 = $el5.nextSibling;
        s2.subscribe(()=>{
            $el6.className = s2.value;
        });
        const $el7 = $el6.nextSibling;
        $el7.style.cssText = s1;
        const $el8 = $el7.nextSibling;
        s1.subscribe(()=>{
            $el8.style.cssText = s1.value;
        });
        const $el9 = $el8.nextSibling;
        $el9.setAttribute("id", n1);
        const $el10 = $el9.nextSibling;
        n1.subscribe(()=>{
            $el10.setAttribute("id", n1.value);
        });
        const $el11 = $el10.nextSibling;
        const $el12 = $el11.firstChild;
        const $el13 = $el12.nextSibling;
        s2.subscribe(()=>{
            $el13.className = s2.value;
        });
        const $el14 = $el11.nextSibling;
        $el14.addEventListener("click", ()=>console.log(el));
        const $el15 = $el14.nextSibling;
        $el15.addEventListener("click", ()=>console.log(1), {});
    });
})();
