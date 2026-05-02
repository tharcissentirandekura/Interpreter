/**
 * Translator from parsed mini-Stata commands into executable Python code.
 *
 * This file contains the code-generation layer of the project. It receives
 * `StataCommand` values, converts expressions such as `log(score)` into NumPy
 * syntax, and emits a complete pandas/statsmodels script as a string.
 *
 * The generated output is what `main.rs` writes to `source_code.py` and runs.
 */
use crate::commands::{
    StataCommand
};
/**
 * Make a dedicated translator from stata to target language
 */
fn valist(vars: &[String]) -> String {
    vars.iter()
        .map(|v| format!("{:?}", v))
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn translate_expr(expr: &str) -> String {
    let expr = expr.trim();

    if expr.starts_with("log(") && expr.ends_with(")") {
        let var = &expr[4..expr.len() - 1];
        return format!("np.log(df[{:?}])", var.trim());
    }

    if expr.starts_with("sqrt(") && expr.ends_with(")") {
        let var = &expr[5..expr.len() - 1];
        return format!("np.sqrt(df[{:?}])", var.trim());
    }

    if expr.starts_with("exp(") && expr.ends_with(")") {
        let var = &expr[4..expr.len() - 1];
        return format!("np.exp(df[{:?}])", var.trim());
    }

    expr.to_string()
}
pub fn convert_to_target(commands:Vec<StataCommand>) ->String{
    let mut target = String::new();
    // import necessary tools
    target.push_str("import pandas as pd\n");
    target.push_str("import numpy as np\n");
    target.push_str("import statsmodels.api as sm\n\n");

    for command in commands{
        match command{
            StataCommand::Use(path) =>{
                target.push_str(&format!("df=pd.read_stata({:?})\n", path));
                target.push_str(&format!("print('Loaded dataset: {}')\n\n", path));
            }
            StataCommand::Summarize(vars) => {
                //collect teh variables to summarize

                if vars.is_empty(){
                    target.push_str(
                        "print(df.describe())\n\n"
                    );
                }else{
                    let variables = vars
                        .iter()
                        .map(|v| format!("{:?}",v))
                        .collect::<Vec<_>>()
                        .join(",");
                    target.push_str(
                        &format!("print(df[[{}]].describe())\n\n",variables)
                    );
                } 
            },

            StataCommand::Describe => {
                //collect teh variables to summarize
                target.push_str("print('--- Dataset Info ---')\n");
                target.push_str("df.info()\n");
                target.push_str("print('\\n--- Preview ---')\n\n");
                target.push_str("print(df.head(10).to_string())\n\n");
            },
            StataCommand::Browse(vars) =>{
                if vars.is_empty(){
                    target.push_str("print(df.describe())\n\n");
                }else{
                    let variables = vars
                        .iter()
                        .map(|v| format!("{:?}",v))
                        .collect::<Vec<_>>()
                        .join(", ");
                    target.push_str(&format!(
                        "print(df[[{}]].describe())\n\n",
                        variables
                    ));
                }
            },
        StataCommand::Regresion { y, regressors, options } => {
            let regs = valist(&regressors);

            let mut subset = vec![y.clone()];
            subset.extend(regressors.clone());

            let subset_target = valist(&subset);
            target.push_str(&format!("reg_df = df.dropna(subset=[{}])\n", subset_target));
            target.push_str(&format!("y = reg_df[{:?}]\n", y));
            target.push_str(&format!("X = reg_df[[{}]]\n", regs));
            target.push_str("X = sm.add_constant(X)\n");
            target.push_str("model = sm.OLS(y, X)\n");

            if options.contains(&"robust".to_string()){
                target.push_str("result =  model.fit(cov_type='HC1') \n");
            }else {
                target.push_str("result = model.fit()\n");
            }
            target.push_str("print(result.summary()) \n\n");

        },

        StataCommand::Generate { name, expr } => {
            let expression = translate_expr(&expr);
            target.push_str(&format!("df[{:?}] = {}\n", name, expression));
            target.push_str(&format!("print('Generated variable: {}')\n\n", name));
        },
    }
    }

    target
}
