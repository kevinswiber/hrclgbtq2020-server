(self.webpackChunklgbtq_state_equality_index=self.webpackChunklgbtq_state_equality_index||[]).push([[225],{3937:function(e,t,n){"use strict";n.r(t);var a=n(8684),r=n(7294),l=n(5313),o=n(920),i=n(4013),m=n(1592),c=n(1128),s=n(1293),u=n(8475),f=n(8063),p=n(9683),d=n(4093),b=n(9395),E=n(9400),y=n(6300),v=n(2760),g=n(2802);function h(e,t){var n="undefined"!=typeof Symbol&&e[Symbol.iterator]||e["@@iterator"];if(n)return(n=n.call(e)).next.bind(n);if(Array.isArray(e)||(n=function(e,t){if(!e)return;if("string"==typeof e)return Z(e,t);var n=Object.prototype.toString.call(e).slice(8,-1);"Object"===n&&e.constructor&&(n=e.constructor.name);if("Map"===n||"Set"===n)return Array.from(e);if("Arguments"===n||/^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(n))return Z(e,t)}(e))||t&&e&&"number"==typeof e.length){n&&(e=n);var a=0;return function(){return a>=e.length?{done:!0}:{done:!1,value:e[a++]}}}throw new TypeError("Invalid attempt to iterate non-iterable instance.\nIn order to be iterable, non-array objects must have a [Symbol.iterator]() method.")}function Z(e,t){(null==t||t>e.length)&&(t=e.length);for(var n=0,a=new Array(t);n<t;n++)a[n]=e[n];return a}var k=(0,o.Z)((function(e){return{formControl:{margin:e.spacing(1),minWidth:120},selectEmpty:{marginTop:e.spacing(2)},tableContainer:{marginTop:20,marginBottom:60},table:{minWidth:650},tableLinks:{marginBottom:40}}})),C=function(e){return e.toLowerCase().replace(" ","-").replace(",","")},S={},w={};t.default=function(e){var t=(0,r.useState)(),n=t[0],o=t[1],Z=e.data.sei.states.edges.map((function(e){return e.node})),A=k();(0,r.useEffect)((function(){var t=e.location.pathname.split("/"),n=t[t.length-1];o("issues-by-state"!==n?n:"")}));for(var _,N=Z.sort((function(e,t){return a.j2p(e.name,t.name)})),j=h(N);!(_=j()).done;){var q=_.value,x=C(q.name);S[x]=q.name,w[q.name]=x}var I=r.createElement(i.Z,{className:A.formControl+" "+g.Y},r.createElement(m.Z,{shrink:!0,htmlFor:"state-select",id:"state-label"},"State"),r.createElement(c.Z,{native:!0,inputProps:{name:"state",id:"state-select"},onChange:function(e){return t=e.target.value,o(t),void(0,l.c4)("/issues/issues-by-state/"+t,{state:t});var t},value:n},r.createElement("option",{key:"none",value:""},"None"),N.map((function(e){return r.createElement("option",{key:e.id,value:w[e.name]},e.name)})))),L=n&&""!==n?Z.find((function(e){return e.name===S[n]})):null;return r.createElement(s.Z,{maxWidth:"md"},r.createElement("h2",null,L&&L.name||""," State policies for LGBTQ+ issues"),I,L&&r.createElement("div",null,r.createElement(v.k,{data:L.issues}),r.createElement(u.Z,{className:A.tableContainer,component:f.Z},r.createElement(p.Z,{className:A.table,"aria-label":"state policy table"},r.createElement(d.Z,null,r.createElement(b.Z,null,r.createElement(E.Z,null,"Issue"),r.createElement(E.Z,null,"Policy"))),r.createElement(y.Z,null,L.issues.map((function(e){return r.createElement(b.Z,{key:e.kind},r.createElement(E.Z,{component:"th",scope:"row"},e.name),r.createElement(E.Z,null,e.policy))}))))),r.createElement(s.Z,{className:A.tableLinks},Z.map((function(e,t){return r.createElement(r.Fragment,null,e.name!==L.name&&r.createElement(l.rU,{to:location.pathname+"?state="+C(e.name)},e.name),e.name===L.name&&e.name,t<Z.length-1&&r.createElement("span",null," | "))})))))}}}]);
//# sourceMappingURL=component---src-pages-issues-issues-by-state-alaska-tsx-17e5e5a045b0f4740a9a.js.map