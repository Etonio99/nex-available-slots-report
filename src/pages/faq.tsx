import {
  BiAbacus,
  BiArrowBack,
  BiMoneyWithdraw,
  BiSolidKey,
  BiSolidUser,
} from 'react-icons/bi';
import Accordion from '../components/accordion';

interface FAQProps {
  navigate: (page: string) => void;
}

const FAQ = (props: FAQProps) => {
  return (
    <div className="max-w-xl m-auto space-y-1">
      <h1 className="text-4xl font-bold mb-4 text-sandstone-300">FAQ</h1>
      <Accordion
        icon={<BiSolidKey />}
        label="What is an API key?"
        text="IDK, ask me later"
      />
      <Accordion
        icon={<BiAbacus />}
        label="How many API calls will this make?"
        text="IDK, ask me later"
      />
      <Accordion
        icon={<BiMoneyWithdraw />}
        label="How much does this cost?"
        text="IDK, ask me later"
      />
      <Accordion
        icon={<BiSolidUser />}
        label="Who made this?"
        text="IDK, ask me later"
      />
      <button
        onClick={() => props.navigate('home')}
        className="flex items-center gap-2 absolute left-4 top-4 px-2 py-1 rounded-sm hover:bg-sandstone-100 text-sandstone-500"
      >
        <BiArrowBack /> Back
      </button>
    </div>
  );
};

export default FAQ;
