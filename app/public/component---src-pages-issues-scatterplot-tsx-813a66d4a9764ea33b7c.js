(self.webpackChunklgbtq_state_equality_index=self.webpackChunklgbtq_state_equality_index||[]).push([[825],{7323:function(e,t,n){"use strict";n.d(t,{i5:function(){return r},xx:function(){return m},ue:function(){return E},p7:function(){return v},V$:function(){return T},_F:function(){return y}});var r,a=n(9756),i=n(7294),o=["orient","range","tickSize"],c=["orient","d3Scale","value","children"],l=["orient","d3Scale","value","children"],u=["orient","tickSize"],s=["orient","tickSize","tickPadding","tickFormat","value"];function d(e){return"translate("+e+",0)"}function f(e){return"translate(0,"+e+")"}!function(e){e[e.TOP=1]="TOP",e[e.RIGHT=2]="RIGHT",e[e.BOTTOM=3]="BOTTOM",e[e.LEFT=4]="LEFT"}(r||(r={}));var m=function(e){var t=e.orient,n=e.range,c=e.tickSize,l=(0,a.Z)(e,o);c=null==c?6:c;var u="undefined"!=typeof window&&window.devicePixelRatio>1?0:.5,s=t===r.TOP||t===r.LEFT?-1:1,d=+n[0]+u,f=+n[n.length-1]+u,m=Object.assign({className:"domain",stroke:"currentColor",d:""},l);return t===r.LEFT||t===r.RIGHT?m.d=c>0?"M"+s*c+","+d+"H"+u+"V"+f+"H"+s*c:"M"+u+","+d+"V"+f:m.d=c>0?"M"+d+","+s*c+"V"+u+"H"+f+"V"+s*c:"M"+d+","+u+"H"+f,i.createElement("path",m)},E=function(e){var t=e.orient,n=e.d3Scale,o=e.value,l=e.children,u=(0,a.Z)(e,c),s="undefined"!=typeof window&&window.devicePixelRatio>1?0:.5,m=t===r.TOP||t===r.BOTTOM?d:f,E=n.copy(),v=Math.max(0,E.bandwidth()-2*s)/2;E.round()&&(v=Math.round(v));var T;return i.createElement("g",Object.assign({className:"tick",opacity:"1",transform:m((T=o,+(E(T)||0)+v+s))},u),l)},v=function(e){var t,n=e.orient,o=e.d3Scale,c=e.value,u=e.children,s=(0,a.Z)(e,l),m="undefined"!=typeof window&&window.devicePixelRatio>1?0:.5,E=n===r.TOP||n===r.BOTTOM?d:f;return i.createElement("g",Object.assign({className:"tick",opacity:"1",transform:E((t=c,+o(t)+m))},s),u)},T=function(e){var t=e.orient,n=e.tickSize,o=void 0===n?6:n,c=(0,a.Z)(e,u),l=t===r.TOP||t===r.LEFT?-1:1,s={stroke:"currentColor"};s[(t===r.LEFT||t===r.RIGHT?"x":"y")+"2"]=l*o;var d=Object.assign({},s,c);return i.createElement("line",d)},y=function(e){var t=e.orient,n=e.tickSize,o=void 0===n?6:n,c=e.tickPadding,l=void 0===c?3:c,u=e.tickFormat,d=e.value,f=(0,a.Z)(e,s),m=t===r.TOP||t===r.LEFT?-1:1,E=t===r.LEFT||t===r.RIGHT?"x":"y",v=Math.max(+o,0)+ +l,T={fill:"currentColor",dy:t===r.TOP?"0em":t===r.BOTTOM?"0.71em":"0.32em"};T[E]=m*v,f=Object.assign(T,f);var y=u?u(d):d;return i.createElement("text",f,y)}},7769:function(e,t,n){"use strict";n.r(t),n.d(t,{default:function(){return s}});var r=n(7294),a=(n(6535),n(9244),n(8684)),i=n(7323);function o(e,t){var n="undefined"!=typeof Symbol&&e[Symbol.iterator]||e["@@iterator"];if(n)return(n=n.call(e)).next.bind(n);if(Array.isArray(e)||(n=function(e,t){if(!e)return;if("string"==typeof e)return c(e,t);var n=Object.prototype.toString.call(e).slice(8,-1);"Object"===n&&e.constructor&&(n=e.constructor.name);if("Map"===n||"Set"===n)return Array.from(e);if("Arguments"===n||/^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(n))return c(e,t)}(e))||t&&e&&"number"==typeof e.length){n&&(e=n);var r=0;return function(){return r>=e.length?{done:!0}:{done:!1,value:e[r++]}}}throw new TypeError("Invalid attempt to iterate non-iterable instance.\nIn order to be iterable, non-array objects must have a [Symbol.iterator]() method.")}function c(e,t){(null==t||t>e.length)&&(t=e.length);for(var n=0,r=new Array(t);n<t;n++)r[n]=e[n];return r}var l=function(e){for(var t,n=400,c=30,l=270,u=20,s=260,d={},f=o(e.issues);!(t=f()).done;){var m=t.value;d[m.id]=m.name}var E=e.states.map((function(e){return e})).sort((function(e,t){return a.j2p(e.id,t.id)})),v=["TRANSGENDER_HEALTHCARE","SCHOOL_ANTI_BULLYING","PUBLIC_ACCOMMODATIONS","MARRIAGE_EQUALITY","HOUSING","HATE_CRIMES","GENDER_MARKER_UPDATES_ON_IDENTIFICATION","EMPLOYMENT","EDUCATION","DISCRIMINATION_IN_CHILD_WELFARE","ANTI_CONVERSION_THERAPY"].map((function(e){return d[e]})),T=a.q2y().domain(v).range([0,n-u]).padding(1),y=a.BYU().domain([-6,6]).range([0,620-l]),O=E.flatMap((function(e){return v.map((function(t){var n=e.issues.find((function(e){return e.name===t}));return{state:e.name,abbreviation:e.id,region:e.region,category:t,status:n.policy,value:n.value}}))})),g=i.i5.BOTTOM,p=i.i5.LEFT;return r.createElement("div",{id:"all-states-scatter-plot"},r.createElement("svg",{viewBox:"0, 0, 620, 400"},r.createElement("g",{stroke:"currentColor",strokeOpacity:"0.1"},y.ticks().map((function(e){return r.createElement("line",{x1:y(e)+s+.5,x2:y(e)+s+.5,y1:c,y2:n-u})})),v.map((function(e){return r.createElement("line",{y1:(T(e)||0)+c,y2:(T(e)||0)+c,x1:s,x2:620})}))),r.createElement("g",{transform:"translate("+s+", "+c+")",fill:"none",fontSize:"12",fontFamily:"sans-serif",textAnchor:"end"},T.domain().map((function(e){return r.createElement(i.ue,{orient:p,d3Scale:T,value:e},r.createElement(i.V$,{orient:p}),r.createElement(i._F,{orient:p,value:e,fontSize:"10"}))}))),r.createElement("g",{transform:"translate("+s+", "+c+")"},O.map((function(e){return r.createElement("g",{key:"datapoint-"+e.state+"-"+e.category},r.createElement("circle",{cx:y(e.value)+.5,cy:T(e.category),r:"3",fill:a.K2I[v.indexOf(e.category)],strokeWidth:"0",fillOpacity:"0.7"}),r.createElement("title",null,e.state+": "+e.category+", "+e.status))}))),r.createElement("g",{transform:"translate("+s+","+(n-u)+")",fill:"none",fontSize:"12",fontFamily:"sans-serif",textAnchor:"start"},y.ticks().map((function(e){return r.createElement(i.p7,{orient:g,d3Scale:y,value:e},r.createElement(i.V$,{orient:g,strokeWidth:"0.7"}),r.createElement(i._F,{orient:g,value:e,fontSize:"8",tickPadding:0}))})))))},u=n(1293),s=function(e){var t=e.data,n=t.sei.states.edges.map((function(e){return e.node})),a=t.sei.issues.edges.map((function(e){return e.node}));return r.createElement("main",null,r.createElement(u.Z,{maxWidth:"lg"},r.createElement("h1",null,"State Equality Index 2020 - All States"),r.createElement(l,{states:n,issues:a})))}}}]);
//# sourceMappingURL=component---src-pages-issues-scatterplot-tsx-813a66d4a9764ea33b7c.js.map