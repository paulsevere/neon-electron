const { hello } = require("../rust-backend/lib");
const $ = require("jquery");
const jQuery = $;
// $("#box").text(hello());
$("#input-form").submit(e => {
  e.preventDefault();
  let text = $("#text-input").val();
  console.log(hello(text || "no input provided", console));
  // window.alert();
});
