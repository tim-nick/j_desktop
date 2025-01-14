<script>
    import { onMount } from 'svelte';

    // https://chatgpt.com/share/67798e15-413c-8002-b2ca-dd6aeff2efb8
  
    let canvas;
    let ctx;
    let drawing = false;
  
    onMount(() => {
      ctx = canvas.getContext('2d');
      ctx.lineWidth = 2;
      ctx.lineCap = 'round';
    });
  
    function startDrawing(event) {
      drawing = true;
      draw(event);
    }
  
    function endDrawing() {
      drawing = false;
      ctx.beginPath();
    }
  
    function draw(event) {
      if (!drawing) return;
      ctx.lineTo(event.clientX - canvas.offsetLeft, event.clientY - canvas.offsetTop);
      ctx.stroke();
      ctx.beginPath();
      ctx.moveTo(event.clientX - canvas.offsetLeft, event.clientY - canvas.offsetTop);
    }
</script>
  
<canvas
    bind:this={canvas}
    width={800}
    height={600}
    on:mousedown={startDrawing}
    on:mouseup={endDrawing}
    on:mousemove={draw}
    style="border:1px solid #000;"
></canvas>