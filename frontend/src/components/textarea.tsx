
type TextAreaProps = {
  callback: (event:React.ChangeEvent<HTMLTextAreaElement>) => void;
  value: string;
};

export const TextArea = ({callback,value}:TextAreaProps) => (
    <textarea 
        className="border rounded-xl w-full h-7/8 resize-none p-10 text-xl" 
        placeholder="Texto do email" 
        onChange={callback}
        value={value}/>
);
