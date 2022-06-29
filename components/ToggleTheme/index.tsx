import Image from "next/image";
import { FC, MouseEventHandler, useEffect, useState } from "react";
import { ActionButton } from "../ActionButton";
import { useTheme } from "next-themes";

export const ToggleTheme: FC = () => {
    const [mounted, setMounted] = useState(false);
    const { theme, setTheme } = useTheme();

    useEffect(() => {
        setMounted(true);
    }, []);

    if (!mounted) {
        return null;
    } else {
        const icon = theme === "dark" ? "/sun.svg" : "/moon.svg";
        return (
            <button
                onClick={() => setTheme(theme === "light" ? "dark" : "light")}
                className='flex items-center justify-center'
            >
                <Image
                    src={icon}
                    alt='light_icon'
                    width={24}
                    height={24}
                />
            </button>
        );
    }
}