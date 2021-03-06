use std::io::Write;

pub struct GanjaGraph<'a> {
    pub title: &'a str,
    pub p: u8,
    pub q: u8,
    pub r: u8,
    pub scale: f64,
    pub grid: bool,
    pub gl: bool,
    pub values: Vec<Vec<Vec<f64>>>,
    pub colors: Vec<u64>,
    pub labels: Vec<Option<&'a str>>,
}

impl Default for GanjaGraph<'_> {
    fn default() -> Self {
        Self {
            title: "ganja-rs",
            p: 0,
            q: 0,
            r: 0,
            scale: 1.0,
            grid: true,
            gl: true,
            values: Vec::new(),
            colors: Vec::new(),
            labels: Vec::new(),
        }
    }
}

impl<'a> GanjaGraph<'a> {
    pub fn add_bivector<T: Into<Vec<f64>>>(
        &mut self,
        bivector: T,
        color: u64,
        label: Option<&'a str>,
    ) {
        self.values.push(vec![bivector.into()]);
        self.colors.push(color);
        self.labels.push(label);
    }

    pub fn add_bivectors<T: Into<Vec<f64>>>(
        &mut self,
        bivectors: Vec<T>,
        color: u64,
        label: Option<&'a str>,
    ) {
        self.values
            .push(bivectors.into_iter().map(|e| e.into()).collect());
        self.colors.push(color);
        self.labels.push(label);
    }

    fn graph_json<W: Write>(&self, out: &mut W) -> std::result::Result<(), std::io::Error> {
        out.write(b"[\n    ")?;
        for (i, v) in self.values.iter().enumerate() {
            if i > 0 {
                out.write(b",\n    ")?;
            }
            out.write(self.colors[i].to_string().as_bytes())?;
            out.write(b",")?;
            if v.len() > 1 {
                out.write(b"[")?;
            }

            let elems = v
                .iter()
                .map(|arr| format!("new Element({:?})", arr))
                .collect::<Vec<_>>();
            out.write(elems.join(",").as_bytes())?;

            if v.len() > 1 {
                out.write(b"]")?;
            }
            if let Some(label) = self.labels[i] {
                out.write(format!(",\"{}\"", label).as_bytes())?;
            }
        }
        out.write(b"\n    ]")?;
        Ok(())
    }

    pub fn graph_html<W: Write>(&self, out: &mut W) -> std::result::Result<(), std::io::Error> {
        let html = format!(
            r#"
<!DOCTYPE html>
<html lang="en" style="height:100%;">
<head>
  <meta charset="UTF-8">
  <title>{title}</title>
  <script type="text/javascript" src="https://enkimute.github.io/ganja.js/ganja.js"></script>
</head>
<body style="position:absolute; top:0; bottom:0; right:0; left:0; overflow:hidden;">
  <script>
  Algebra({p},{q},{r},()=>{{
  var canvas = this.graph("#,
            title = self.title,
            p = self.p,
            q = self.q,
            r = self.r,
        );
        out.write(html.as_bytes())?;

        self.graph_json(out)?;

        let html = format!(
            r#",
    {{conformal:{conformal},gl:{gl},grid:{grid},scale:{scale},useUnnaturalLineDisplayForPointPairs:true}});
  canvas.style.width = '100vw';
  canvas.style.height = '100vh';
  document.body.appendChild(canvas);
  }});
  </script>
</body>
</html>
"#,
            conformal = (self.q != 0).to_string(),
            grid = self.grid.to_string(),
            scale = self.scale.to_string(),
            gl = self.gl.to_string(),
        );
        out.write(html.as_bytes())?;
        Ok(())
    }
}
