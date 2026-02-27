import { useState } from "react";

export type MultiSelectItem = {
    label: string,
    description: string,
    key: string,
    checked?: boolean,
    onChange?: (key: string, checked: boolean) => void,
}

interface MultiSelectProps {
    items: MultiSelectItem[],
}

const MultiSelect = (props: MultiSelectProps) => {
    return (
        <div>
            <div className="px-2 py-1 bg-sandstone-50">
                <p className="text-lg font-bold">Select which locations you want to query</p>
                <p className="text-sandstone-400 text-sm">This may save you on API calls</p>
            </div>
            <div className="overflow-y-auto max-h-64">
                <ul className="p-2 space-y-2 bg-sandstone-200 rounded-md">
                    {
                        props.items.map(item =>
                            <MultiSelectItem key={item.key} label={item.label} description={item.description} checked={item.checked ?? false} />
                        )
                    }
                </ul>
            </div>
        </div>
    );
}

interface MultiSelectItemProps {
    label: string,
    description: string,
    key: string,
    checked: boolean,
    onChange?: (key: string, checked: boolean) => void,
}

const MultiSelectItem = (props: MultiSelectItemProps) => {
    const [checked, setChecked] = useState(props.checked);

    const toggle = () => {
        const newValue = !checked;
        setChecked(!newValue);
        props.onChange?.(props.key, newValue);
    }

    return (
        <li onClick={toggle} className={`${checked ? "" : "brightness-80"} bg-sandstone-50 grid grid-cols-[40px_1fr] rounded-sm overflow-hidden shadow shadow-sandstone-300 cursor-pointer`}>
            <div className="grid place-items-center bg-sandstone-100">
                <input type="checkbox" checked={checked} className="pointer-events-none" />
            </div>
            <div className="pl-2">
                <p className="relative top-0.5">{props.label}</p>
                <p className="text-sandstone-300 relative bottom-0.5">{props.description}</p>
            </div>
        </li>
    )
}

export default MultiSelect;