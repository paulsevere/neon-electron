const { exec } = require("child_process");

process.chdir("rust-backend");

exec("neon build", (error, stdout, stderr) => {
  if (error) {
    console.log(error);
  } else {
    console.log(stdout, stderr);
  }
});
