console.log("syntax check");
ace.define("ace/mode/celltail_highlight_rules", [
    "require",
    "exports",
    "module",
    "ace/lib/oop", "ace/mode/text_highlight_rules"
],
    function (require, exports, module) {
        console.log("highlight rules");

        var oop = require("../lib/oop");
        var TextHighlightRules = require("./text_highlight_rules").TextHighlightRules;

        var CellTailHighlightRules = function () {

            var keywords = (
                "fn|I|INPUT|D|DEBUG|O|OUTPUT"
            );

            var builtinConstants = (
                "N|false|true"
            );

            var builtinFunctions = (
                ""
            );

            //var futureReserved = "";
            var keywordMapper = this.createKeywordMapper({
                "invalid.deprecated": "debugger",
                "support.function": builtinFunctions,
                "variable.language": "self|cls",
                "constant.language": builtinConstants,
                "keyword": keywords
            }, "identifier");

            var decimalInteger = "(?:(?:[1-9]\\d*)|(?:0))";
            var integer = "(?:" + decimalInteger + ")";

            this.$rules = {
                "start": [{
                    token: "comment",
                    regex: "#.*$"
                }, {
                    token: "string",           // " string
                    regex: '"',
                    next: "qstring"
                }, {
                    token: "string",           // Character
                    regex: "'.'",
                }, {
                    token: "keyword.operator",
                    regex: "\\+|\\-|\\*|\\/|\\||\\&|%|\\^|\\.\\."
                }, {
                    token: "punctuation",
                    regex: ",|:|;|\\->|\\+=|\\-=|\\*=|\\/=|\\/\\/=|%=|@=|&=|\\|=|^=|>>=|<<=|\\*\\*="
                }, {
                    token: "paren.lparen",
                    regex: "[\\[\\(\\{]"
                }, {
                    token: "paren.rparen",
                    regex: "[\\]\\)\\}]"
                }, {
                    token: ["keyword", "text", "entity.name.function"],
                    regex: "(def|class)(\\s+)([\\u00BF-\\u1FFF\\u2C00-\\uD7FF\\w]+)"
                }, {
                    token: "text",
                    regex: "\\s+"
                }, {
                    include: "constants"
                }],
                "qstring": [
                    {
                        token: "string",
                        regex: "\"",
                        next: "start",
                    },
                    {
                        token: "string",
                        regex: "[^\"]",
                        next: "qstring"
                    }

                ],
                "constants": [{
                    token: "constant.numeric", // integer
                    regex: integer + "\\b"
                }, {
                    token: keywordMapper,
                    regex: "[a-zA-Z_$][a-zA-Z0-9_$]*\\b"
                }]
            };
            this.normalizeRules();
        };

        oop.inherits(CellTailHighlightRules, TextHighlightRules);

        exports.CellTailHighlightRules = CellTailHighlightRules;
    }
);
ace.define("ace/mode/celltail", ["require", "exports", "module",
    "ace/lib/oop", "ace/mode/text", "ace/mode/celltail_highlight_rules",
], function (require, exports, module) {
    console.log("inner mode");

    var oop = require("../lib/oop");
    var TextMode = require("./text").Mode;
    var IniHighlightRules = require("ace/mode/celltail_highlight_rules").CellTailHighlightRules;
    // var IniFoldMode = require("./folding/ini").FoldMode;

    var Mode = function () {
        this.HighlightRules = IniHighlightRules;
        // this.foldingRules = new IniFoldMode();
        this.$behaviour = this.$defaultBehaviour;
    };
    oop.inherits(Mode, TextMode);

    (function () {
        this.lineCommentStart = "#";
        this.blockComment = null;
        this.$id = "ace/mode/ini";
    }).call(Mode.prototype);

    exports.Mode = Mode;
});


(function () {
    ace.require(["ace/mode/celltail"], function (m) {
        if (typeof module == "object" && typeof exports == "object" && module) {
            module.exports = m;
        }
    });
})();

console.log("syntax.js");
