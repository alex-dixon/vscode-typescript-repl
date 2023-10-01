import test from "ava";

import { transformSync, evaluableSpans, transformSyncRegular } from "../index.js";

const makeSpanTestInput = (s) => {
  const pos = s.indexOf("|");
  return [s.split("|").join(""), pos];
};

test("should rewrite named import if the name is used as a value", (t) => {
  const output = transformSync(`import {foo} from 'bar';foo`);
  console.log("the output", output);
  t.deepEqual(output,
    { code: `var { foo  } = require("bar");\nfoo;\nfoo;\n`, isAsync: false }
  );
});
test("should rewrite named import even if it looks like the name is not used", (t) => {
  const output = transformSync(`import {foo} from 'bar';`);
  console.log("the output", output);
  t.deepEqual(output,
    {
      code: `var { foo  } = require("bar");\nfoo;\n`,
      isAsync: false
    }
  );
});
test("should always emit valid javascript (remove typescript only stuff)", (t) => {
  const output = transformSync(`const foo: number = 42;`);
  console.log("the output", output);
  t.deepEqual(output,
    {
      code: `var foo = 42;\n`, isAsync: false
    }
  );
});
test("no 'use strict'", (t) => {
  const output = transformSync(`export const foo = 42;`);
  console.log("the output", output);
  t.deepEqual(output.code.startsWith("\"use strict\""), false);
});

test("should allow class redef", (t) => {
  const output = transformSync(`class Hey {};`);
  console.log("the output", output);
  t.deepEqual(output.code.startsWith("\"use strict\""), false);
});


test("simple export rewrite", (t) => {
  const output = transformSync(`export const foo = 42;`);
  console.log("the output", output);
  t.deepEqual(output,
    {
      code: `Object.defineProperty(exports, "__esModule", {
    value: true,
    configurable: true
});
Object.defineProperty(exports, "foo", {
    enumerable: true,
    get: function() {
        return foo;
    },
    configurable: true
});
var foo = 42;\n`,
      isAsync: false

    }
  );
});

test("evaluable spans", (t) => {
  const output = evaluableSpans(
    `export const foo = 42;`,
    `export const foo = 42;`.length
  );
  console.log("the output", output);
  t.deepEqual(!!output.spans, true);
});


test("evaluable spans - function", (t) => {
  const input = makeSpanTestInput(
    `
export const foo = (a: number, b: number): Promise<E.Either<unknown,never>> =>
  4|2;`.trimStart()
  );
  const output = evaluableSpans(
    ...input
  );
  console.log("the output", output);
  t.deepEqual(output.spans[output.spans.length - 1], {
    // is this off off?
    // end: 85,
    // start: 83,
    end: 84,
    start: 82,
    type: "NumericLiteral"
  });
});

test("evaluable spans - object literal", (t) => {
  const input = makeSpanTestInput(
    `
const what = {|
    foo: {bar:'baz'},
    ok: true,
};`.trimStart()
  );
  const output = evaluableSpans(
    ...input
  );
  console.log("the output", output);
  t.deepEqual(output.spans[output.spans.length - 1], {
    end: 53,
    start: 14,
    type: "ObjectExpression"
  });
});


test("evaluable spans - property path", (t) => {
  const input = makeSpanTestInput(
    `a.b|.c.d`.trimStart()
  );
  const output = evaluableSpans(
    ...input
  );
  console.log("the output", output);
  t.deepEqual(output.spans[output.spans.length - 1], {
    end: 4,
    start: 1,
    type: "MemberExpression"
  });
});

test("evaluable spans - function param", (t) => {
  const input = makeSpanTestInput(
    `const f = (a|: number) => {}`.trimStart()
  );
  const output = evaluableSpans(
    ...input
  );
  console.log("the output", output);
  t.deepEqual(output.spans[output.spans.length - 1], {
    end: 13,
    start: 12,
    type: "Identifier"
  });
});


test("evaluable spans - should not panic on invalid code", (t) => {
  const input = makeSpanTestInput(
    `i am invalid co|de`.trimStart()
  );
  const output = evaluableSpans(
    ...input
  );
  console.log("the output", output);
  t.deepEqual(output.spans, []);
});

test("evaluable spans - should do something reasonable for call exprs", (t) => {
  const input = makeSpanTestInput(
    `foo.bar.baz('heyo')|`.trimStart()
  );
  const output = evaluableSpans(
    ...input
  );
  console.log("the output", output);
  t.deepEqual(output.spans, [{
    end: 20,
    start: 1,
    type: "ExpressionStatement"
  }, { end: 20, start: 1, type: "CallExpression" }]);
});

test("top-level await", (t) => {
  const input = `const foo = async () => 42;
  const bar = await foo()`;
  const output = transformSync(input);
  console.log("the output", output.code.toString());
  t.deepEqual(output, {
    code: `var foo, bar;
(async ()=>{
    foo = async ()=>42;
    bar = await foo();
})();
`,
    isAsync: true,
  });
});

test("top-level await return last if await", (t) => {
  const input = `const foo = async () => 42;
  await foo()`;
  const output = transformSync(input);
  console.log("the output", output.code.toString());
  t.deepEqual(output, {
   code: `var foo;
(async ()=>{
    foo = async ()=>42;
    return await foo();
})();
`,
   isAsync: true,
 })
});

test("just rewrite typescript to javascript",(t)=> {
  const input = `const foo = async (): Promise<number> => 42;
  await foo()`;
  const output = transformSyncRegular(input);
  const output2 = transformSync(input);
  console.log("the output", output.code.toString());
  t.deepEqual(output, {
    code: `const foo = async ()=>42;
await foo();
`
  })

})

test ("redecl when swc uses a loop for multiple exports", (t) => {
    const input = `
    export const foo = 42;
    export const bar = 43;
    function _export (obj, name, getter) {
        if (name in obj) {
            } 
            }
    `;
    const output = transformSync(input);
    console.log("the output", output.code.toString());
    t.deepEqual(output, {
        code: `Object.defineProperty(exports, "__esModule", {
    value: true,
    configurable: true
});
function _export(target, all) {
    for(var name in all)Object.defineProperty(target, name, {
        enumerable: true,
        get: all[name],
        configurable: true
    });
}
_export(exports, {
    foo: function() {
        return foo;
    },
    bar: function() {
        return bar;
    }
});
var foo = 42;
var bar = 43;
function _export(obj, name, getter) {
    if (name in obj) {}
}
`,
        isAsync: false
    })

})