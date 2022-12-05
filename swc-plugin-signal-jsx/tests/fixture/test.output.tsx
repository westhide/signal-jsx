import {component$} from "@westhide/tai";

const s1 = 1;
const s2 = 2;
const n1 = 1;
const el = 1;
const $cmpt = 1;
const c1 = (() => {
    const $cmpt1 = Comp({
        prop1: s1
    });
    return component$(`<div >${$cmpt1.html}<div1 >''""</div1><div2 > </div2><div3 class="cls1"></div3><div4 ></div4><div4 ></div4><div4 ></div4><div4 ></div4><div4 ></div4><div4 ></div4><div6 ><div7 ></div7><div8 ></div8></div6><div5 ></div5><div5 ></div5></div>`, ($el) => {
        el.value = $el;
        $el.className = $cmpt;
        const $el1 = $el.firstChild;
        $cmpt1.patch($el1);
        const $el2 = toSibling($el1, 2);
        s1.subscribe(() => {
            $el2.textContent = s1.value;
        });
        const $el3 = $el2.nextSibling;
        n1.value = $el3;
        const $el4 = $el3.nextSibling;
        $el4.className = s1;
        const $el5 = $el4.nextSibling;
        s2.subscribe(() => {
            $el5.className = s2.value;
        });
        const $el6 = $el5.nextSibling;
        $el6.style.cssText = s1;
        const $el7 = $el6.nextSibling;
        s1.subscribe(() => {
            $el7.style.cssText = s1.value;
        });
        const $el8 = $el7.nextSibling;
        $el8.setAttribute("id", n1);
        const $el9 = $el8.nextSibling;
        n1.subscribe(() => {
            $el9.setAttribute("id", n1.value);
        });
        const $el10 = $el9.nextSibling;
        const $el11 = $el10.firstChild;
        const $el12 = $el11.nextSibling;
        s2.subscribe(() => {
            $el12.className = s2.value;
        });
        const $el13 = $el10.nextSibling;
        $el13.addEventListener("click", () => console.log(el));
        const $el14 = $el13.nextSibling;
        $el14.addEventListener("click", () => console.log(1), {});
    });
})();
