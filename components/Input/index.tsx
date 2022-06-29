import { Dispatch, FC, SetStateAction, useEffect, useState } from "react";
import styles from './Input.module.css';
import { useTheme} from "next-themes";

export const Input: FC<{
	label: string;
	value: string;
    setValue: Dispatch<SetStateAction<string>>;
    type?: string;
}> = ({ label, value, setValue, type}) => {
	const [mounted, setMounted] = useState(false);
	const { theme } = useTheme();

	useEffect(() => {
		setMounted(true);
    }, []);

	if (!mounted) {
		return null;
	} else {
		return (
			<label className={`${styles.input_container}`}>
				{type === "textarea" ? (
					<textarea
						className={`${styles.input}  ${
							theme === "dark" && styles.input_dark
						} ${type === "textarea" && styles.input_textarea}`}
						placeholder=' '
						required
						value={value}
						onChange={(e) => setValue(e.target.value)}
					/>
				) : (
					<input
						className={`${styles.input}  ${
							theme === "dark" && styles.input_dark
						}`}
						type={type || "text"}
						placeholder=' '
						required
						value={value}
						onChange={(e) => setValue(e.target.value)}
					/>
				)}
				<span
					className={`${styles.input_label} ${
						theme === "dark" && styles.input_label_dark
					}  ${type === "textarea" && styles.input_label_textarea}`}
				>
					{label}
				</span>
			</label>
		);
	}
};
