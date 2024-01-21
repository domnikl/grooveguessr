import { useState } from "react";

export default function useTimer(duration: number, onFinished: () => void) {
  const [remaining, setRemaining] = useState<number>(duration);

  const start = function () {
    let r = duration;

    const interval = setInterval(() => {
      if (r > 0) {
        r -= 1;
        setRemaining(r);
      } else {
        onFinished();
        clearInterval(interval);
        setRemaining(0);
      }
    }, 1000);
  };

  return { start, remaining };
}
