import { useEffect, useRef, useState } from "react";

type NumberScrollerProps = {
  number: number;
};

export default function ({ number }: NumberScrollerProps) {
  const domRef = useRef<HTMLDivElement>(null);
  const saveNum = useRef(number);
  const [showNumber, setShowNumber] = useState<number>(number);
  const scroll = () => {
    setShowNumber(number);
    const frames = new KeyframeEffect(
      domRef.current,
      [{ transform: `translateY(0px)` }, { transform: `translateY(-28px)` }],
      {
        duration: 300,
        fill: "forwards",
      }
    );

    const animation = new Animation(frames);
    animation.play();
    animation.finished.then(() => {
      saveNum.current = number;
    });
  };

  useEffect(() => {
    scroll();
  }, [number]);

  return (
    <div className="w-[10px] rounded-[4px] text-center h-[30px]  relative overflow-hidden">
      <div
        ref={domRef}
        className="absolute right-0 text-center top-0 left-0 transition-transform"
      >
        <div className="w-full h-[30px] leading-[30px] text-center ">
          {saveNum.current}
        </div>
        <div>{showNumber}</div>
        <div className="w-full text-center h-[30px] leading-[30px]">{0}</div>
      </div>
    </div>
  );
}
