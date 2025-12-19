import type {ButtonClick} from '@customTypes/events';

type ButtonProps = {
  onClick: (event:ButtonClick) => void;
};

export const Button = ({onClick}:ButtonProps) => (
    <button className="w-full rounded-xl p-5 text-xl bg-red-500 text-white" onClick={onClick}>Send</button>
)