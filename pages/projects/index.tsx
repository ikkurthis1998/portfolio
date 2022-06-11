import type { NextPage } from "next";

const Projects: NextPage = () => {
	return <ProjectsTitle />;
};

export default Projects;

function ProjectsTitle() {
	return (
		<h1 className='text-3xl font-bold underline text-center'>
			Projects!
		</h1>
	);
}
