module.exports = {
  siteMetadata: {
    siteUrl: "https://www.yourdomain.tld",
    title: "2020 LGBTQ+ Rights Visualizations",
  },
  plugins: [
    "gatsby-theme-material-ui",
    "gatsby-plugin-postcss",
    {
      resolve: "gatsby-plugin-google-analytics",
      options: {
        trackingId: "278226599",
      },
    },
    {
      resolve: "gatsby-source-graphql",
      options: {
        typeName: "SEI",
        fieldName: "sei",
        url: "https://hrc-lgbtq-2020.herokuapp.com/graphql",
      },
    },
  ],
};
