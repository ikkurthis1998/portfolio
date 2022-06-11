import { FC, useEffect, useState } from "react";
import styles from './Input.module.css';
import { useTheme} from "next-themes";

export const Input: FC = () => {
    const [mounted, setMounted] = useState(false);
	const { theme, setTheme } = useTheme();

	useEffect(() => {
		setMounted(true);
    }, []);
    
    if (!mounted) {
        return null;
    } else {
        return (
            <label className={styles.input_container}>
                <input
                    className={styles.input}
                    type='text'
                    placeholder=' '
                    required
                />
                <span
                    className={`${styles.input_label} ${theme === 'dark' && styles.input_label_dark}`}
                >
                    Basic
                </span>
            </label>
        );
    }
};
