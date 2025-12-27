import type {ButtonClick} from '@customTypes/events';

type ButtonProps = {
  onClick: (event:ButtonClick) => void;
};

export const Button = ({onClick}:ButtonProps) => (
    <button 
    	className="w-full rounded-xl p-5 h-1/8 text-xl bg-red-500 text-white cursor-pointer" 
	onClick={onClick}>
	Enviar
    </button>
)
