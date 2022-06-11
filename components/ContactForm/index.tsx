import { FormEvent, useState } from "react";
import { Input } from "../Input";

export const ContactForm = () => {

    const [name, setName] = useState("");
    const [email, setEmail] = useState("");
    const [message, setMessage] = useState("");
    const [error, setError] = useState("");
    const [success, setSuccess] = useState("");

    const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        if (!name || !email || !message) {
            setError("Please fill all the fields");
            return;
        }
        try {
            const response = await fetch("/api/addContactRequest", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    name,
                    email,
                    message
                })
            });
            if (response.status === 200) {
                setName("");
                setEmail("");
                setMessage("");
                setError("");
                setSuccess("Your request has been submitted successfully");
            } else {
                setError("Something went wrong");
            }
        } catch (e) {
            setError("Something went wrong");
        }
    };

    return (
		<div className='flex flex-col justify-center'>
			<h1 className='text-3xl font-bold'>
				Contact Me
            </h1>
            <div>
                {/* Error message will be displayed here */}
                {error && <p className='text-red-500'>{error}</p>}
            </div>
            <div>
                {/* Success message will be displayed here */}
                {success && <p className='text-green-500'>{success}</p>}
            </div>
			<form
				className='flex flex-col justify-center gap-1'
				onSubmit={handleSubmit}
            >
                <Input />
				<label className='' htmlFor='name'>
					Name
				</label>
				<input
					className='w-full p-2'
					type='text'
					name='name'
                    id='name'
                    onChange={(e) => setName(e.target.value)}
                    placeholder='Your name'
                    value={name}
				/>
				<label className='' htmlFor='email'>
					Email
				</label>
				<input
					className='w-full p-2'
					type='email'
					name='email'
                    id='email'
                    onChange={(e) => setEmail(e.target.value)}
                    placeholder='Your email'
                    value={email}
				/>
				<label className='' htmlFor='message'>
					Message
				</label>
				<textarea
					className='w-full p-2'
					name='message'
                    id='message'
                    onChange={(e) => setMessage(e.target.value)}
                    placeholder='Your message'
                    value={message}
				/>
				<button
					className='w-full p-2 bg-blue-500 text-white'
					type='submit'
				>
					Send
				</button>
			</form>
		</div>
	);
}