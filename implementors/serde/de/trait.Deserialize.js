(function() {var implementors = {};
implementors["egg"] = [{"text":"impl&lt;'de, L, D&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"egg/struct.EClass.html\" title=\"struct egg::EClass\">EClass</a>&lt;L, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt;,&nbsp;</span>","synthetic":false,"types":["egg::eclass::EClass"]},{"text":"impl&lt;'de, L:&nbsp;<a class=\"trait\" href=\"egg/trait.Language.html\" title=\"trait egg::Language\">Language</a>, N:&nbsp;<a class=\"trait\" href=\"egg/trait.Analysis.html\" title=\"trait egg::Analysis\">Analysis</a>&lt;L&gt;&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"egg/struct.EGraph.html\" title=\"struct egg::EGraph\">EGraph</a>&lt;L, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N::<a class=\"associatedtype\" href=\"egg/trait.Analysis.html#associatedtype.Data\" title=\"type egg::Analysis::Data\">Data</a>: for&lt;'a&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;L: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt;,&nbsp;</span>","synthetic":false,"types":["egg::egraph::EGraph"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"egg/enum.Justification.html\" title=\"enum egg::Justification\">Justification</a>","synthetic":false,"types":["egg::explain::Justification"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"egg/struct.SymbolLang.html\" title=\"struct egg::SymbolLang\">SymbolLang</a>","synthetic":false,"types":["egg::language::SymbolLang"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"egg/struct.Id.html\" title=\"struct egg::Id\">Id</a>","synthetic":false,"types":["egg::Id"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()