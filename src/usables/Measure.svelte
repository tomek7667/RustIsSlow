<script>
  import { invoke } from "@tauri-apps/api";
  let contents;
  let rustResult = "";
  let jsResult = "";
  let rustOut = "";
  let jsOut = "";
  const measureRustPerformance = async () => {
    let start = Date.now();
    invoke("measure_perfomance").then((result) => {
      rustOut = result[0];
      rustResult = `${result[1]} ms`;
    });
  };

  const measure_performanceJS = () => {
    return new Promise((resolve) => {
      let sss = 0;
      for (let i = 0; i < 10000000; i++) {
        if (i % 2 == 0) {
          sss += i;
        }
        if ((i % 3 == 0 && i * i < 500) || i / 2 == 0) {
          sss *= 2;
        }
        if ((i % 5 == 0 && i * i < 500) || i / 2 == 0) {
          sss /= 2;
        }
      }
      return resolve(sss);
    });
  };

  const measureJavascriptPerformance = async () => {
    let start = Date.now();
    measure_performanceJS().then((result) => {
      let end = Date.now();
      jsOut = result;
      jsResult = `${end - start} ms`;
    });
  };
</script>

<div>
  <button class="rustResult" on:click={measureRustPerformance}
    >Measure rust performance</button
  >
  <button class="javascriptResult" on:click={measureJavascriptPerformance}
    >Measure js performance</button
  >
  {#if contents}
    <p>{contents}</p>
  {/if}
</div>
<div class="rustResult">RUST: {rustResult} :{rustOut}</div>
<div class="javascriptResult">JS: {jsResult} : {jsOut}</div>

<style>
  .rustResult {
    width: 50%;
    float: left;
    text-align: center;
    font-size: 24px;
  }
  .javascriptResult {
    width: 50%;
    float: right;
    text-align: center;
    font-size: 24px;
  }
</style>
