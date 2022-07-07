
import('../pkg').then(init).catch(console.error);

function init(wasm) {
  const canvas = document.getElementById('drawing');
  const ctx = canvas.getContext('2d');
  let displayRange;
  let draggingInfo;
  const redrow = (x1, y1, x2, y2) => {
    canvas.width = canvas.clientWidth;
    canvas.height = canvas.clientHeight;
    wasm.draw(
      ctx, x1, y1, x2, y2,
      (adjusted) => { displayRange = { ...adjusted }}
    );
  };
  window.addEventListener('resize', () => {
    const {x1, y1, x2, y2} = displayRange;
    redrow(x1, y1, x2, y2)
  });
  const pointerDown = (evnet) => {
    canvas.setPointerCapture(event.pointerId)
    canvas.addEventListener('pointerup', pointerUp)
    canvas.addEventListener('pointermove', pointerMove)
    canvas.removeEventListener('pointerdown', pointerDown)
  };
  const pointerMove = (evnet) => {
    const rect = canvas.getBoundingClientRect(),
      left = event.clientX - rect.left,
      top = event.clientY - rect.top;
    if (draggingInfo) {
      draggingInfo = {
        ...draggingInfo,
        left: Math.min(draggingInfo.startLeft, left),
        top: Math.min(draggingInfo.startTop, top),
        width: Math.abs(draggingInfo.startLeft - left),
        height: Math.abs(draggingInfo.startTop - top),
      }
    } else {
      draggingInfo = {
        startLeft: left,
        startTop: top,
        left,
        top,
        width: 0,
        height: 0,
      }
    }
  }
  const pointerUp = (evnet) => {
    if (displayRange && draggingInfo) {
      const { left, width, top, height } = draggingInfo;
      const draggedDistance = Math.pow(width, 2) + Math.pow(height, 2)
      if (draggedDistance >= 500) {
        const { x1, y1, x2, y2 } = displayRange;
        const xr = (x2 - x1) / canvas.clientWidth
        const yr = (y2 - y1) / canvas.clientHeight
        redrow(
          x1 + left * xr,
          y1 + top * yr,
          x1 + (left + width) * xr,
          y1 + (top + height) * yr,
        )
      }
      draggingInfo = null;
    }
    canvas.releasePointerCapture(event.pointerId)
    canvas.removeEventListener('pointermove', pointerMove)
    canvas.addEventListener('pointerdown', pointerDown)
  }
  canvas.addEventListener('pointerdown', pointerDown)
  redrow(-2, -1.5, 1, 1.5);
}
