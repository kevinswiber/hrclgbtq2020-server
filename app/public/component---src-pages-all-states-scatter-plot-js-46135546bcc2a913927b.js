(self.webpackChunklgbtq_state_equality_index=self.webpackChunklgbtq_state_equality_index||[]).push([[22],{1293:function(e,t,n){"use strict";var r=n(2122),a=n(1253),i=n(6156),o=n(7294),c=n(5505),s=n(4621),u=n(1664),l=o.forwardRef((function(e,t){var n=e.classes,i=e.className,s=e.component,l=void 0===s?"div":s,d=e.disableGutters,m=void 0!==d&&d,f=e.fixed,p=void 0!==f&&f,g=e.maxWidth,v=void 0===g?"lg":g,x=(0,a.Z)(e,["classes","className","component","disableGutters","fixed","maxWidth"]);return o.createElement(l,(0,r.Z)({className:(0,c.Z)(n.root,i,p&&n.fixed,m&&n.disableGutters,!1!==v&&n["maxWidth".concat((0,u.Z)(String(v)))]),ref:t},x))}));t.Z=(0,s.Z)((function(e){return{root:(0,i.Z)({width:"100%",marginLeft:"auto",boxSizing:"border-box",marginRight:"auto",paddingLeft:e.spacing(2),paddingRight:e.spacing(2),display:"block"},e.breakpoints.up("sm"),{paddingLeft:e.spacing(3),paddingRight:e.spacing(3)}),disableGutters:{paddingLeft:0,paddingRight:0},fixed:Object.keys(e.breakpoints.values).reduce((function(t,n){var r=e.breakpoints.values[n];return 0!==r&&(t[e.breakpoints.up(n)]={maxWidth:r}),t}),{}),maxWidthXs:(0,i.Z)({},e.breakpoints.up("xs"),{maxWidth:Math.max(e.breakpoints.values.xs,444)}),maxWidthSm:(0,i.Z)({},e.breakpoints.up("sm"),{maxWidth:e.breakpoints.values.sm}),maxWidthMd:(0,i.Z)({},e.breakpoints.up("md"),{maxWidth:e.breakpoints.values.md}),maxWidthLg:(0,i.Z)({},e.breakpoints.up("lg"),{maxWidth:e.breakpoints.values.lg}),maxWidthXl:(0,i.Z)({},e.breakpoints.up("xl"),{maxWidth:e.breakpoints.values.xl})}}),{name:"MuiContainer"})(l)},1664:function(e,t,n){"use strict";n.d(t,{Z:function(){return a}});var r=n(2192);function a(e){if("string"!=typeof e)throw new Error((0,r.Z)(7));return e.charAt(0).toUpperCase()+e.slice(1)}},7300:function(e,t,n){"use strict";n.d(t,{i5:function(){return u},xx:function(){return p},dR:function(){return g},V$:function(){return v},_F:function(){return x}});var r=n(9756),a=n(7294),i=["orient","scale","tickSize"],o=["orient","scale","value","line","text"],c=["orient","tickSize"],s=["orient","tickSize","tickPadding","tickFormat","ticks","value","scale"],u=Object.freeze({TOP:1,RIGHT:2,BOTTOM:3,LEFT:4});function l(e){return"translate("+e+",0)"}function d(e){return"translate(0,"+e+")"}function m(e){return function(t){return+e(t)}}function f(e,t){return t=Math.max(0,e.bandwidth()-2*t)/2,e.round()&&(t=Math.round(t)),function(n){return+e(n)+t}}var p=function(e){var t=e.orient,n=e.scale,o=e.tickSize,c=(0,r.Z)(e,i);o=null==o?6:o;var s="undefined"!=typeof window&&window.devicePixelRatio>1?0:.5,u=1===t||4===t?-1:1,l=n.range(),d=+l[0]+s,m=+l[l.length-1]+s;return(c=Object.assign({className:"domain",stroke:"currentColor"},c)).d=4===t||2===t?o>0?"M"+u*o+","+d+"H"+s+"V"+m+"H"+u*o:"M"+s+","+d+"V"+m:o>0?"M"+d+","+u*o+"V"+s+"H"+m+"V"+u*o:"M"+d+","+s+"H"+m,a.createElement("path",c)},g=function(e){var t=e.orient,n=e.scale,i=e.value,c=e.line,s=e.text,u=(0,r.Z)(e,o),p="undefined"!=typeof window&&window.devicePixelRatio>1?0:.5,g=1===t||3===t?l:d,v=(n.bandwidth?f:m)(n.copy(),p);return a.createElement("g",Object.assign({className:"tick",opacity:"1",transform:g(v(i)+p)},u),c,s)},v=function(e){var t=e.orient,n=e.tickSize,i=void 0===n?6:n,o=(0,r.Z)(e,c),s=1===t||4===t?-1:1,u={stroke:"currentColor"};return u[(4===t||2===t?"x":"y")+"2"]=s*i,o=Object.assign(u,o),a.createElement("line",o)},x=function(e){var t=e.orient,n=e.tickSize,i=void 0===n?6:n,o=e.tickPadding,c=void 0===o?3:o,u=e.tickFormat,l=e.ticks,d=void 0===l?[]:l,m=e.value,f=e.scale,p=(0,r.Z)(e,s),g=1===t||4===t?-1:1,v=4===t||2===t?"x":"y",x=null==u?f.tickFormat?f.tickFormat.apply(f,d):function(e){return e}:u,E=Math.max(+i,0)+ +c,b={fill:"currentColor",dy:1===t?"0em":3===t?"0.71em":"0.32em"};return b[v]=g*E,p=Object.assign(b,p),a.createElement("text",p,x(m))}},1905:function(e,t,n){"use strict";n.r(t),n.d(t,{default:function(){return l}});var r=n(7294),a=(n(6535),n(9244),n(8684)),i=n(7300);function o(e,t){var n="undefined"!=typeof Symbol&&e[Symbol.iterator]||e["@@iterator"];if(n)return(n=n.call(e)).next.bind(n);if(Array.isArray(e)||(n=function(e,t){if(!e)return;if("string"==typeof e)return c(e,t);var n=Object.prototype.toString.call(e).slice(8,-1);"Object"===n&&e.constructor&&(n=e.constructor.name);if("Map"===n||"Set"===n)return Array.from(e);if("Arguments"===n||/^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(n))return c(e,t)}(e))||t&&e&&"number"==typeof e.length){n&&(e=n);var r=0;return function(){return r>=e.length?{done:!0}:{done:!1,value:e[r++]}}}throw new TypeError("Invalid attempt to iterate non-iterable instance.\nIn order to be iterable, non-array objects must have a [Symbol.iterator]() method.")}function c(e,t){(null==t||t>e.length)&&(t=e.length);for(var n=0,r=new Array(t);n<t;n++)r[n]=e[n];return r}var s=function(e){for(var t,n=400,c=30,s=270,u=20,l=260,d={},m=o(e.issues);!(t=m()).done;){var f=t.value;d[f.id]=f.name}var p=e.states.map((function(e){return e})).sort((function(e,t){return a.j2p(e.id,t.id)})),g=["TRANSGENDER_HEALTHCARE","SCHOOL_ANTI_BULLYING","PUBLIC_ACCOMMODATIONS","MARRIAGE_EQUALITY","HOUSING","HATE_CRIMES","GENDER_MARKER_UPDATES_ON_IDENTIFICATION","EMPLOYMENT","EDUCATION","DISCRIMINATION_IN_CHILD_WELFARE","ANTI_CONVERSION_THERAPY"].map((function(e){return d[e]})),v=a.q2y().domain(g).range([0,n-u]).padding(1),x=a.BYU().domain([-6,6]).range([0,620-s]),E=p.flatMap((function(e){return g.map((function(t){var n=e.issues.find((function(e){return e.name===t}));return{state:e.name,abbreviation:e.id,region:e.region,category:t,status:n.policy,value:n.value}}))})),b=i.i5.BOTTOM,k=i.i5.LEFT;return r.createElement("div",{id:"all-states-scatter-plot"},r.createElement("svg",{viewBox:"0, 0, 620, 400"},r.createElement("g",{stroke:"currentColor",strokeOpacity:"0.1"},x.ticks().map((function(e){return r.createElement("line",{x1:x(e)+l+.5,x2:x(e)+l+.5,y1:c,y2:n-u})})),g.map((function(e){return r.createElement("line",{y1:v(e)+c,y2:v(e)+c,x1:l,x2:620})}))),r.createElement("g",{transform:"translate("+l+", "+c+")",fill:"none",fontSize:"12",fontFamily:"sans-serif",textAnchor:"end"},v.domain().map((function(e){return r.createElement(i.dR,{orient:k,scale:v,value:e,tickSize:"6",line:r.createElement(i.V$,{orient:k}),text:r.createElement(i._F,{orient:k,scale:v,value:e,fontSize:"10"})})}))),r.createElement("g",{transform:"translate("+l+", "+c+")"},E.map((function(e){return r.createElement("g",{key:"datapoint-"+e.state+"-"+e.category},r.createElement("circle",{cx:x(e.value)+.5,cy:v(e.category),r:"3",fill:a.K2I[g.indexOf(e.category)],strokeWidth:"0",fillOpacity:"0.7"}),r.createElement("title",null,e.state+": "+e.category+", "+e.status))}))),r.createElement("g",{transform:"translate("+l+","+(n-u)+")",fill:"none",fontSize:"12",fontFamily:"sans-serif",textAnchor:"start"},x.ticks().map((function(e){return r.createElement(i.dR,{orient:b,scale:x,value:e,line:r.createElement(i.V$,{orient:b,strokeWidth:"0.7"}),text:r.createElement(i._F,{orient:b,scale:x,value:e,fontSize:"8",tickPadding:"0"})})})))))},u=n(1293),l=function(e){var t=e.data,n=t.sei.states.edges.map((function(e){return e.node})),a=t.sei.issues.edges.map((function(e){return e.node}));return r.createElement("main",null,r.createElement(u.Z,{maxWidth:"lg"},r.createElement("h1",null,"State Equality Index 2020 - All States"),r.createElement(s,{states:n,issues:a})))}}}]);
//# sourceMappingURL=component---src-pages-all-states-scatter-plot-js-46135546bcc2a913927b.js.map