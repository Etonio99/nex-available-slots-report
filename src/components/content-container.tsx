interface ContentContainerProps {
  children: React.ReactNode;
}

const ContentContainer = (props: ContentContainerProps) => {
  return (
    <div className="max-h-full h-full overflow-y-scroll rounded-md shadow-xl shadow-sandstone-950/20 p-4 bg-sandstone-25 border border-sandstone-300 pb-32">
      {props.children}
    </div>
  );
};

export default ContentContainer;
