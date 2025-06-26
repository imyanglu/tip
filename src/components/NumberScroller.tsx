import { useMemo } from "react";
import ScrollItem from "./ScrollItem";

type NumberScrollerProps = {
  number: string;
  before?: string;
  after?: string;
};

const NumberScroller = ({ number, before, after }: NumberScrollerProps) => {
  console.log(number);
  const numArr = useMemo(() => {
    return number.split("").map((num) => num);
  }, [number]);

  return (
    <div className="flex gap-x-[4px] items-center">
      {before}
      {numArr.map((i, idx) => {
        if (/\w/.test(i)) {
          return <ScrollItem key={idx} number={Number(i)} />;
        }
        return <>{i}</>;
      })}
      {after}
    </div>
  );
};
export default NumberScroller;
