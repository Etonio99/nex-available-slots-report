import { BiArrowBack } from 'react-icons/bi';

interface HelpProps {
  navigate: (page: string) => void;
}

const Help = (props: HelpProps) => {
  return (
    <div className="max-w-xl m-auto space-y-1 h-full flex flex-col">
      <h1 className="text-4xl font-bold mb-4 text-sandstone-300">Help</h1>
      <p>HELP ME!</p>

      <button onClick={() => props.navigate('faq')}>Visit the FAQ</button>

      <button
        onClick={() => props.navigate('home')}
        className="flex items-center gap-2 absolute left-4 top-4 px-2 py-1 rounded-sm hover:bg-sandstone-100 text-sandstone-500"
      >
        <BiArrowBack /> Back
      </button>
    </div>
  );
};

export default Help;
