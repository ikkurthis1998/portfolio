import type { NextPage } from "next";

const Blogs: NextPage = () => {
	return <BlogsTitle />;
};

export default Blogs;

function BlogsTitle() {
	return (
		<h1 className='text-3xl font-bold underline text-center'>Blogs!</h1>
	);
}
