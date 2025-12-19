
type TextAreaProps = {
  callback: (event:React.ChangeEvent<HTMLTextAreaElement>) => void;
  value: string;
};

export const TextArea = ({callback,value}:TextAreaProps) => (
    <textarea 
        className="border rounded-xl w-full h-full resize-none p-10 text-xl" 
        placeholder="Email Text" 
        onChange={callback}
        value={value}/>
);